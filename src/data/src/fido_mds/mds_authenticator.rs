use crate::database::{Cache, DB};
use crate::entity::webauthn::{force_mds_attestation, verify_mds_attestation};
use crate::fido_mds::mds_entry::MdsEntrySimple;
use crate::rauthy_config::RauthyConfig;
use ahash::{HashMap, HashMapExt};
use hiqlite::macros::FromRow;
use hiqlite::params;
use rauthy_common::constants::IDX_WEBAUTHN_MDS_CA_LIST;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::collections::BTreeMap;
use tracing::error;
use uuid::Uuid;
use webauthn_rs::prelude::{AttestationCaList, AttestationCaListBuilder};

/// An MDS entry joined with its distinct root certificates, as read from the database.
#[derive(Debug)]
pub struct MdsAuthenticator {
    pub entry: MdsEntrySimple,
    pub certs: Vec<Vec<u8>>,
}

impl MdsAuthenticator {
    /// Every authenticator in the dataset, each joined with its distinct root certificates.
    async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let entries = MdsEntrySimple::find_all().await?;
        let certs = MdsAaguidCertRow::find_all().await?;
        let mut certs_map: HashMap<Vec<u8>, Vec<Vec<u8>>> = HashMap::with_capacity(certs.len());
        for cert in certs {
            if let Some(v) = certs_map.get_mut(&cert.aaguid) {
                v.push(cert.cert_der);
            } else {
                certs_map.insert(cert.aaguid, vec![cert.cert_der]);
            }
        }

        let mut authenticators: Vec<Self> = Vec::with_capacity(entries.len());
        for entry in entries {
            let Some(certs) = certs_map.get(&entry.aaguid) else {
                error!("No MDS certs found for MdsEntry - this should never happen");
                continue;
            };
            authenticators.push(Self {
                entry,
                // multiple entries share AAGUIDs
                certs: certs.clone(),
            });
        }

        Ok(authenticators)
    }

    pub async fn get_ca_list() -> Result<Option<AttestationCaList>, ErrorResponse> {
        if let Some(json) = DB::hql()
            .get::<_, _, Option<String>>(Cache::App, IDX_WEBAUTHN_MDS_CA_LIST)
            .await?
        {
            return Ok(json.map(|j| serde_json::from_str(&j).unwrap()));
        }

        let ca_list = Self::build_ca_list().await?;

        let json = ca_list
            .as_ref()
            .map(|list| serde_json::to_string(list).unwrap());

        // App cache on purpose. It's being cleared on startup so that config changes are reflected.
        DB::hql()
            .put(Cache::App, IDX_WEBAUTHN_MDS_CA_LIST, &json, Some(12 * 3600))
            .await?;

        Ok(ca_list)
    }

    async fn build_ca_list() -> Result<Option<AttestationCaList>, ErrorResponse> {
        let mut authenticators = Self::find_all().await?;
        if force_mds_attestation() {
            // In this case, we can pre-filter the list
            let config = &RauthyConfig::get().vars.webauthn;
            authenticators.retain(|a| {
                verify_mds_attestation(
                    &a.entry,
                    &config.force_passkey_cert_level,
                    &config.force_passkey_protection,
                    &config.force_passkey_attachment,
                )
                .is_ok()
            });
        }
        if authenticators.is_empty() {
            error!("Filtered FIDO MDS dataset is empty - no passkey attestation can be verified");
            return Ok(None);
        }

        let mut builder = AttestationCaListBuilder::new();
        for auth in authenticators {
            let Ok(aaguid) = Uuid::from_slice(&auth.entry.aaguid) else {
                error!("Invalid AAGUID in MDS certs set");
                continue;
            };

            for cert in &auth.certs {
                if let Err(err) = builder.insert_device_der(
                    cert,
                    aaguid,
                    auth.entry.description.clone(),
                    BTreeMap::new(),
                ) {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        format!("Failed to insert MDS root certificate: {err:?}"),
                    ));
                }
            }
        }
        Ok(Some(builder.build()))
    }
}

#[derive(Debug, FromRow, FromPgRow)]
struct MdsAaguidCertRow {
    aaguid: Vec<u8>,
    cert_der: Vec<u8>,
}

impl MdsAaguidCertRow {
    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = r#"
SELECT e.aaguid, c.cert_der
FROM fido_mds_entry_certs e
JOIN fido_mds_certs c ON c.hash = e.cert_hash
ORDER BY e.aaguid, c.hash"#;

        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            // at the time of implementation, we have 716 distinct rows in `fido_mds_entry_certs`
            DB::pg_query(sql, &[], 768).await?
        };

        Ok(res)
    }
}
