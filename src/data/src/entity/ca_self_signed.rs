use crate::database::DB;
use crate::rauthy_config::RauthyConfig;
use cryptr::EncValue;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{deserialize, serialize};
use rauthy_error::ErrorResponse;
use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::ops::{Add, Sub};
use std::time::Duration;
use time::OffsetDateTime;
use tokio::task;
use tracing::info;

static SELF_SIGNED_LT_SECS: u64 = 3600 * 72;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfSignedCA {
    key_pair: Vec<u8>,
    nbf: OffsetDateTime,
    exp: OffsetDateTime,
    cert: String,
}

impl From<hiqlite::Row<'_>> for SelfSignedCA {
    fn from(mut row: hiqlite::Row) -> Self {
        let bytes: Vec<u8> = row.get("data");
        deserialize(bytes.as_ref()).unwrap()
    }
}

impl From<tokio_postgres::Row> for SelfSignedCA {
    fn from(row: tokio_postgres::Row) -> Self {
        let bytes: Vec<u8> = row.get("data");
        deserialize(bytes.as_ref()).unwrap()
    }
}

impl SelfSignedCA {
    pub async fn find_or_generate_new() -> Result<Self, ErrorResponse> {
        let now = OffsetDateTime::now_utc();

        let sql = "SELECT data FROM config WHERE id = 'ca'";
        let slf: Option<Self> = if is_hiqlite() {
            DB::hql().query_map_optional(sql, params!()).await?
        } else {
            DB::pg_query_opt(sql, &[]).await?
        };
        if let Some(slf) = slf
            && slf.exp > now.add(Duration::from_secs(3600))
        {
            info!("Found already existing Self-Signed CA");
            return Ok(slf);
        }

        info!("Generating new Self-Signed CA");
        let nbf = now.sub(Duration::from_secs(60));
        let exp = now.add(Duration::from_secs(24 * 3600 * 3650));

        let kp = Self::generate_key_pair().await;
        let key_pair = EncValue::encrypt(kp.serialize_pem().as_bytes())?
            .into_bytes()
            .to_vec();

        let slf = Self {
            key_pair,
            nbf,
            exp,
            cert: Self::params(nbf, exp).self_signed(&kp).unwrap().pem(),
        };
        let slf_bytes = serialize(&slf)?;

        let sql = r#"
INSERT INTO config (id, data)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE
SET data = $2
"#;
        if is_hiqlite() {
            DB::hql().execute(sql, params!("ca", slf_bytes)).await?;
        } else {
            DB::pg_execute(sql, &[&"ca", &slf_bytes]).await?;
        }

        Ok(slf)
    }

    pub async fn create_end_entity(&self) -> Result<SelfSignedCert, ErrorResponse> {
        let pub_url = &RauthyConfig::get().vars.server.pub_url;
        let name = if let Some((name, _)) = pub_url.rsplit_once(":") {
            name
        } else {
            pub_url
        };

        let mut params = CertificateParams::new(vec![name.to_string()]).unwrap();
        params.distinguished_name.push(DnType::CommonName, name);
        params.use_authority_key_identifier_extension = true;
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params
            .extended_key_usages
            .push(ExtendedKeyUsagePurpose::ServerAuth);

        let now = OffsetDateTime::now_utc();
        params.not_before = now.sub(Duration::from_secs(60));
        let exp = now.add(Duration::from_secs(SELF_SIGNED_LT_SECS));
        params.not_after = exp;

        let params_ca = Self::params(self.nbf, self.exp);
        let key_pair_ca = self.key_pair()?;
        let iss = Issuer::from_params(&params_ca, &key_pair_ca);

        let key_pair = Self::generate_key_pair().await;
        let cert = params.signed_by(&key_pair, &iss).unwrap();

        let mut cert_chain_pem = cert.pem();
        writeln!(cert_chain_pem, "{}", self.cert)?;

        Ok(SelfSignedCert {
            key_pem: key_pair.serialize_pem(),
            cert_chain_pem,
            exp,
        })
    }

    #[inline]
    async fn generate_key_pair() -> KeyPair {
        task::spawn_blocking(|| rcgen::KeyPair::generate().unwrap())
            .await
            .unwrap()
    }

    fn key_pair(&self) -> Result<KeyPair, ErrorResponse> {
        let dec = EncValue::try_from(self.key_pair.clone())?.decrypt()?;
        let s = String::from_utf8_lossy(dec.as_ref());
        Ok(KeyPair::from_pem(s.as_ref()).unwrap())
    }

    fn params(nbf: OffsetDateTime, exp: OffsetDateTime) -> CertificateParams {
        let mut params = CertificateParams::default();

        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params
            .distinguished_name
            .push(DnType::CommonName, "ca.rauthy.local");
        params
            .distinguished_name
            .push(DnType::OrganizationName, "Rauthy Self-Signed");
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params.key_usages.push(KeyUsagePurpose::KeyCertSign);
        params.key_usages.push(KeyUsagePurpose::CrlSign);
        params.not_before = nbf;
        params.not_after = exp;

        params
    }
}

pub struct SelfSignedCert {
    pub key_pem: String,
    pub cert_chain_pem: String,
    pub exp: OffsetDateTime,
}
