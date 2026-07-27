use crate::cli_args::ArgsGenConfig;
use crate::utils::StdError;
use crate::utils::stdin::{PromptPassword, read_line_stdin, read_line_stdin_yes};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, PasswordHasher, Version};
use chrono::Utc;
use cidr::IpCidr;
use colored::Colorize;
use cryptr::EncKeys;
use cryptr::utils::secure_random_alnum;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use tokio::fs;
use validator::Validate;
use zeroize::Zeroize;

#[derive(Default)]
struct GenConfig {
    // Cluster
    node_id_from_k8s: bool,
    nodes: Vec<String>,
    cluster_tls_auto_certificates: Option<bool>,

    // Database + Backups
    use_hiqlite: Option<bool>,
    cache_storage_disk: Option<bool>,
    s3_config: Option<S3Config>,
    pg_config: Option<PGConfig>,

    // Bootstrap Admin
    admin_email: String,
    password_hash: Option<String>,

    // Server
    use_https: bool,
    tls_generate_self_signed: Option<bool>,
    tls_cert_path: Option<String>,
    tls_key_path: Option<String>,
    proxy_mode: bool,
    trusted_proxies: Vec<IpCidr>,
    pub_url: String,

    // Webauthn
    rp_id: String,
    rp_origin: String,
    rp_name: String,

    // E-Mail
    smtp_url: String,
    smtp_username: String,
    smtp_password: String,
    smtp_from: String,

    // Addons
    admin_force_mfa: bool,
    user_self_delete_enable: Option<bool>,
    user_registration_enable: Option<bool>,
}

#[derive(Default)]
struct S3Config {
    url: String,
    bucket: String,
    region: String,
    path_style: bool,
    key: String,
    secret: String,
}

#[derive(Default)]
struct PGConfig {
    host: String,
    user: String,
    password: String,
    db_name: String,
    pg_tls_no_verify: bool,
}

impl Display for GenConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"## The full reference config can be found here:
## https://sebadob.github.io/rauthy/config/config.html
"#
        )?;

        writeln!(
            f,
            r#"
[bootstrap]
# If set, the email of the default admin will be changed during
# the initialization of an empty production database.
#
# default: 'admin@localhost'
# overwritten by: BOOTSTRAP_ADMIN_EMAIL
admin_email = '{}'"#,
            self.admin_email
        )?;

        if let Some(hash) = self.password_hash.as_ref() {
            writeln!(
                f,
                r#"
# If set, this will take the Argon2ID hashed password during the
# initialization of an empty production database. If both
# `password_plain` and `pasword_argon2id` are set, the hashed
# version will always be prioritized.
#
# default: random -> see logs on first start
# overwritten by: BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID
pasword_argon2id = '{}'"#,
                hash
            )?;
        }

        if self.node_id_from_k8s {
            writeln!(
                f,
                r#"
[cluster]
# Can be set to 'k8s' to try to split off the node id from the hostname
# when Hiqlite is running as a StatefulSet inside Kubernetes.
#
# default: unset
# overwritten by: HQL_NODE_ID_FROM
node_id_from = 'k8s'"#
            )?;
        } else {
            writeln!(
                f,
                r#"
[cluster]
# The node id must exist in the nodes and there must always be
# at least a node with ID 1
# Will be ignored if `node_id_from = k8s`
#
# At least `node_id_from` or `node_id` are required.
#
# default: 0 (invalid)
# overwritten by: HQL_NODE_ID
node_id = 1"#
            )?;
        }
        writeln!(f, "\nnodes = [")?;
        for node in &self.nodes {
            writeln!(f, "    '{node}',")?;
        }
        writeln!(f, "]")?;

        if let Some(tls) = self.cluster_tls_auto_certificates {
            writeln!(
                f,
                r#"
# The `tls_auto_certificates` will generate self-signed TLS
# certificates for internal Raft and API traffic. Clients will
# simply not validate the certificates for ease of use because
# they don't have to. They do a 3-way handshake anyway, which
# validates both client and server without the secret ever being
# sent over the network.
#
# If you specify specific certificates with either `tls_raft_*` or
# `tls_api_*`, they will be used instead.
#
# default: false
# overwritten by: HQL_TLS_AUTO_CERTS
tls_auto_certificates = {}"#,
                tls
            )?;
        }

        if let Some(cache_storage_disk) = self.cache_storage_disk {
            writeln!(
                f,
                r#"
# Set to `false` to store Cache WAL files + Snapshots in-memory only.
# If you run a Cluster, a Node can re-sync cache data after a restart.
# However, if you restart too quickly or shut down the whole cluster,
# all your cached data will be gone.
# In-memory only significantly increases the throughput though, so it
# depends on your needs, what you should prefer.
#
# default: true
# overwritten by: HQL_CACHE_STORAGE_DISK
cache_storage_disk = {}"#,
                cache_storage_disk
            )?;
        }

        writeln!(
            f,
            r#"
# Secrets for Raft internal authentication as well as for the API.
# These must be at least 16 characters long, and you should provide
# different ones for both variables.
#
# default: not set - required
# overwritten by: HQL_SECRET_RAFT
secret_raft = '{}'
# default: not set - required
# overwritten by: HQL_SECRET_API
secret_api = '{}'"#,
            secure_random_alnum(48),
            secure_random_alnum(48)
        )?;

        if let Some(s3) = &self.s3_config {
            writeln!(
                f,
                r#"
# Access values for the S3 bucket where backups will be pushed to.
# overwritten by: HQL_S3_URL
s3_url = '{}'
# overwritten by: HQL_S3_BUCKET
s3_bucket = '{}'
# overwritten by: HQL_S3_REGION
s3_region = '{}'
# overwritten by: HQL_S3_PATH_STYLE
s3_path_style = {}
# overwritten by: HQL_S3_KEY
s3_key = '{}'
# overwritten by: HQL_S3_SECRET
s3_secret = '{}'"#,
                s3.url, s3.bucket, s3.region, s3.path_style, s3.key, s3.secret
            )?;
        }

        writeln!(f, "\n[database]")?;

        if let Some(hiqlite) = self.use_hiqlite {
            writeln!(
                f,
                r#"# Hiqlite is the default database for Rauthy.
# You can opt-out and use Postgres instead by setting this
# value to `false`. Rauthy will then read the `pg_*` values
# below to establish a Postgres connection.
#
# default: true
# overwritten by: HIQLITE
hiqlite = {}"#,
                hiqlite
            )?;
        }
        if let Some(pg) = &self.pg_config {
            writeln!(
                f,
                r#"# If you set `hiqlite = false` and want to use Postgres as your
# database, you need to set the following variables.
# These will be ignored as long as `hiqlite = true`.
#
# overwritten by: PG_HOST
pg_host = '{}'
# default: 5432
# overwritten by: PG_PORT
pg_port = 5432
# overwritten by: PG_USER
pg_user = '{}'
# overwritten by: PG_PASSWORD
pg_password = '{}'
# default: rauthy
# overwritten by: PG_DB_NAME
pg_db_name = '{}'

# If your database uses a self-signed certificate, which cannot
# be verified, you might want to set this to `true`.
#
# default: false
# overwritten by: PG_TLS_NO_VERIFY
pg_tls_no_verify = {}"#,
                pg.host, pg.user, pg.password, pg.db_name, pg.pg_tls_no_verify
            )?;
        }

        writeln!(
            f,
            r#"
[email]
# Rauthy will force TLS and try a downgrade to STARTTLS, if
# TLS fails. It will never allow an unencrypted connection.
# You might want to set `SMTP_DANGER_INSECURE=true` if you
# need this for local dev.
#
# overwritten by: SMTP_URL
smtp_url = '{}'
# optional, default will be used depending on TLS / STARTTLS
# overwritten by: SMTP_PORT
#smtp_port =
# overwritten by: SMTP_USERNAME
smtp_username = '{}'
# overwritten by: SMTP_PASSWORD
smtp_password = '{}'
# Format: "Rauthy <rauthy@localhost>"
# default: "Rauthy <rauthy@localhost>"
# overwritten by: SMTP_FROM
smtp_from = '{}'"#,
            self.smtp_url, self.smtp_username, self.smtp_password, self.smtp_from
        )?;

        let keys = EncKeys::generate_with_id(Utc::now().date_naive().to_string()).unwrap();
        let keys_b64 = keys.keys_as_b64_vec();
        writeln!(
            f,
            r#"
[encryption]
# You need to define at least one valid encryption key.
# These keys are used in various places, like for instance
# encrypting confidential client secrets in the database, or
# encryption cookies, and so on.
#
# The first part until the first `/` is the key ID.
# The ID must match '^[a-zA-Z0-9:_-]{{2,20}}$'
#
# The key itself begins after the first `/` has been found.
# The key must be exactly 32 bytes long, encoded as base64.
#
# You can find a more detailed explanation on how to generate
# keys in the documentation:
# 1. https://sebadob.github.io/rauthy/getting_started/k8s.html#create-and-apply-secrets
# 2. https://sebadob.github.io/rauthy/config/encryption.html
#
# You can provide multiple keys to make things like key
# rotation work. Be careful with removing old keys. Make sure
# that all secrets have been migrated beforehand.
# You can find a utility in the Admin UI to do this for you.
#
# overwritten by: ENC_KEYS - single String, \n separated values
keys = ['{}']

# This identifies the key ID from the `ENC_KEYS` list that
# should actively be used for new encryptions.
#
# overwritten by: ENC_KEY_ACTIVE
key_active = '{}'"#,
            keys_b64.first().unwrap(),
            keys.enc_key_active
        )?;

        writeln!(
            f,
            r#"
[events]
# The E-Mail address event notifications should be sent to.
#
# overwritten by: EVENT_EMAIL
email = '{}'"#,
            self.admin_email
        )?;

        writeln!(
            f,
            r#"
[mfa]
# If 'true', MFA for an account must be enabled to access the
# rauthy admin UI.
#
# default: true
# overwritten by: ADMIN_FORCE_MFA
admin_force_mfa = {}"#,
            self.admin_force_mfa
        )?;

        let scheme = if self.use_https { "https" } else { "http" };
        writeln!(
            f,
            r#"
[server]
# The scheme to use locally, valid values:
# http | https | http_https | unix_http | unix_https
# For more details about the UNIX domain socket, check out its
# documentation page.
#
# default: http_https
# overwritten by: LISTEN_SCHEME
scheme = '{}'

# The Public URL of the whole deployment
# The LISTEN_SCHEME + PUB_URL must match the HTTP ORIGIN HEADER
# later on, which is especially important when running Rauthy
# behind a reverse proxy. In case of a non-standard port (80/443),
# you need to add the port to the PUB_URL
#
# default: not set - mandatory
# overwritten by: PUB_URL
pub_url = '{}'"#,
            scheme, self.pub_url
        )?;

        if !self.trusted_proxies.is_empty() {
            writeln!(
                f,
                r#"
# When rauthy is running behind a reverse proxy, set to true
#
# default: false
# overwritten by: PROXY_MODE
proxy_mode = {}

# A list of trusted proxy CIDRs. When `proxy_mode = true`
# or `peer_ip_header_name` is set, these are mandatory to
# be able to extract the real client IP properly and safely
# to prevent IP header spoofing. All requests witha
# different source will be blocked.
#
# default: []
# overwritten by: TRUSTED_PROXIES - single String, \n separated values
trusted_proxies = [
"#,
                self.proxy_mode
            )?;
            for cidr in &self.trusted_proxies {
                writeln!(f, "    '{}',", cidr)?;
            }
            writeln!(f, "]")?;
        }

        let mut tls_added = false;
        if let Some(cert_path) = &self.tls_cert_path
            && let Some(key_path) = &self.tls_key_path
        {
            tls_added = true;

            writeln!(
                f,
                r#"
[tls]
## UI + API TLS

# Overwrite the path to the TLS certificate file in PEM
# format for rauthy
#
#default: 'tls/tls.crt'
# overwritten by: TLS_CERT
cert_path = '{}'

# Overwrite the path to the TLS private key file in PEM
# format for rauthy. If the path / filename ends with '.der',
# rauthy will parse it as DER, otherwise as PEM.
#
# default: 'tls/tls.key'
# overwritten by: TLS_KEY
key_path = '{}'"#,
                cert_path, key_path
            )?;
        }

        if let Some(self_signed) = self.tls_generate_self_signed {
            if !tls_added {
                writeln!(f, "\n[tls]")?;
            }
            writeln!(
                f,
                r#"
# If set to `true`, Rauthy will generate self-signed TLS certs and copy
# them into `tls/self_signed_cert.pem` and `tls/self_signed_key.pem`.
# It will also IGNORE any `cert_path` / `key_path`.
#
# CAUTION: If set to `true`, it will delete existing files:
# - `tls/self_signed_cert.pem`
# - `tls/self_signed_key.pem`
#
# This should only be used for testing and never in production!
#
# default: false
# overwritten by: TLS_GENERATE_SELF_SIGNED
generate_self_signed = {}"#,
                self_signed
            )?;
        }

        if let Some(self_del) = self.user_self_delete_enable {
            writeln!(
                f,
                r#"
[user_delete]
# You can enable user self-deletion via the Account Dashboard.
# It is disabled by default, because especially if you use things like
# SCIM, the deletion of a user might trigger a series of events which
# will delete other important data as well, that might be linked to a
# user account, and you want to clean up manually before a user is being
# fully deleted.
#
# default: false
# overwritten by: USER_ENABLE_SELF_DELETE
enable_self_delete = {}"#,
                self_del
            )?;
        }

        if let Some(user_reg) = self.user_registration_enable {
            writeln!(
                f,
                r#"
[user_registration]
# If the User Registration endpoint should be accessible by anyone.
# If not, an admin must create each new user.
#
# default: false
# overwritten by: OPEN_USER_REG
enable = {}"#,
                user_reg
            )?;
        }

        writeln!(
            f,
            r#"
[webauthn]
# The 'Relaying Party (RP) ID' - effective domain name.
#
# CAUTION: When this changes, already registered devices will
# stop working and users cannot log in anymore!
#
# default: 'localhost'
# overwritten by: RP_ID
rp_id = '{}'

# Url containing the effective domain name.
#
# DEV: If you want to test Webauthn via the Svelte DEV UI,
# change the port number to :5173.
#
# !!! CAUTION: Must ALWAYS include the port number !!!
#
# default: 'http://localhost:8080'
# overwritten by: RP_ORIGIN
rp_origin = '{}'

# Non-critical RP Name
# Has no security properties and may be changed without issues
#
# default: 'Rauthy IAM'
# overwritten by: RP_NAME
rp_name = '{}'"#,
            self.rp_id, self.rp_origin, self.rp_name
        )?;

        Ok(())
    }
}

impl GenConfig {
    async fn prompt() -> Result<Self, StdError> {
        println!("Rauthy config file generation.");
        println!("Please answer the following questions.");

        let mut slf = Self::default();

        ///////////////////////
        // Cluster
        ///////////////////////
        println!(
            "\nDo you want to run a '{}' instance or a '{}' cluster?",
            "single".green(),
            "HA".green()
        );
        loop {
            let v = read_line_stdin().await?.trim().to_lowercase();
            if v == "single" {
                slf.node_id_from_k8s = false;
                slf.nodes
                    .push("1 localhost:8100 localhost:8200".to_string());

                println!(
                    "\nSince it only really makes sense for single-instance deployments, \
                do you want to keep the cache in-memory only?"
                );
                slf.cache_storage_disk = Some(!read_line_stdin_yes().await?);

                break;
            } else if v == "ha" {
                println!("\nHA deployment - will it run in Kubernetes later on?");
                if read_line_stdin_yes().await? {
                    slf.node_id_from_k8s = true;
                } else {
                    slf.node_id_from_k8s = false;
                    println!(
                        "Please note that you must adjust the `cluster.node_id` \
                        in the end for each node individually."
                    );
                };

                println!(
                    r#"
The recommended set for HA is 3 nodes. Do NOT run 2 nodes.
It makes no sense. Rauthy uses the Raft consensus algorithm
under the hood which already needs a quorum. We assume you
want to run 3 nodes. Provide the 3 IPs or DNS names for
each node 1 by 1:"#
                );

                println!("Node 1: ");
                slf.nodes.push(read_line_stdin().await?.trim().to_string());
                println!("Node 2: ");
                slf.nodes.push(read_line_stdin().await?.trim().to_string());
                println!("Node 3: ");
                slf.nodes.push(read_line_stdin().await?.trim().to_string());

                println!("\nDo you want to enable auto-TLS for in-cluster traffic?");
                if read_line_stdin_yes().await? {
                    slf.cluster_tls_auto_certificates = Some(true);
                } else {
                    slf.cluster_tls_auto_certificates = Some(false);
                }

                break;
            } else {
                eprintln!("{}", "Invalid input - try again.".red());
            }
        }

        ///////////////////////
        // Database + Backups
        ///////////////////////
        println!(
            "\nWhat kind of database do you want to use? The default '{}' or '{}'?",
            "hiqlite".green(),
            "postgres".green()
        );
        loop {
            let v = read_line_stdin().await?.trim().to_lowercase();
            if v == "hiqlite" {
                println!("\nDo you want to configure an S3 bucket for encrypted backups?");
                if read_line_stdin_yes().await? {
                    let mut cfg = S3Config::default();

                    println!("S3 URL:");
                    cfg.url = read_line_stdin().await?;
                    println!("Bucket Name:");
                    cfg.bucket = read_line_stdin().await?;
                    println!("Region:");
                    cfg.region = read_line_stdin().await?;
                    println!("Use Path-Style? (Minio, Garage, ...)");
                    cfg.path_style = read_line_stdin_yes().await?;
                    println!("Access Key ID:");
                    cfg.key = read_line_stdin().await?;
                    cfg.secret = PromptPassword::default()
                        .prompt("Access Key Secret:".to_string())
                        .await?;

                    slf.s3_config = Some(cfg);
                }

                break;
            } else if v == "postgres" {
                slf.use_hiqlite = Some(false);
                let mut cfg = PGConfig::default();

                println!("\nPostgres Host:");
                cfg.host = read_line_stdin().await?;
                println!("Postgres User:");
                cfg.user = read_line_stdin().await?;
                cfg.password = PromptPassword::default()
                    .prompt("Postgres Password: ".to_string())
                    .await?;
                println!("Postgres Database Name:");
                cfg.db_name = read_line_stdin().await?;
                println!("Skip TLS verification for Postgres connections?");
                cfg.pg_tls_no_verify = read_line_stdin_yes().await?;

                slf.pg_config = Some(cfg);
                break;
            } else {
                eprintln!("{}", "Invalid input - try again.".red());
            }
        }

        ///////////////////////
        // Database + Backups
        ///////////////////////
        println!("\nE-Mail address for the initial Admin account: ");
        loop {
            slf.admin_email = read_line_stdin().await?;
            if (ValidateEmail {
                email: slf.admin_email.as_str(),
            })
            .validate()
            .is_ok()
            {
                break;
            } else {
                eprintln!("{}", "Invalid E-Mail address, please try again.".red());
            }
        }
        println!(
            "\nDo you want to bootstrap an existing password?\n\
    A random one will be generated otherwise on first start."
        );
        if read_line_stdin_yes().await? {
            let msg = r#"

To be as secure as possible, Rauthy comes with a default
password policy you must match. At least:
- 14 characters long
- 1 lowercase char
- 1 uppercase char
- 1 digit
"#
            .to_string();

            let policy = PromptPassword::default();

            let mut plain;
            loop {
                plain = policy.prompt_validated(&msg).await?;

                let plain_confirm = policy
                    .prompt("Please enter the same password again for confirmation".to_string())
                    .await?;
                if plain == plain_confirm {
                    break;
                }
                eprintln!(
                    "{}",
                    "The two passwords do not match, please try again.".red()
                );
            }

            println!("Hashing password ...");
            let params = argon2::ParamsBuilder::new()
                .m_cost(131_072)
                .t_cost(4)
                .p_cost(8)
                .build()?;
            let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
            let salt = SaltString::generate(&mut OsRng);
            let hashed = argon2.hash_password(plain.as_bytes(), &salt)?;

            plain.zeroize();

            slf.password_hash = Some(hashed.to_string());
        }

        ///////////////////////
        // Server
        ///////////////////////
        println!("\nShould Rauthy use HTTPS?");
        slf.use_https = read_line_stdin_yes().await?;
        if slf.use_https {
            println!(
                "\nShould Rauthy generate '{}'-generated TLS certificates or do you want to provide '{}' ones?",
                "auto".green(),
                "existing".green()
            );
            let v = read_line_stdin().await?.trim().to_lowercase();
            if v == "auto" {
                slf.tls_generate_self_signed = Some(true);
            } else if v == "existing" {
                println!(
                    r#"
The default path's where Rauthy will look for TLS certificates are
'tls/tls.crt' and 'tls/tls.key'. Do you want to overwrite these?"#
                );
                if read_line_stdin_yes().await? {
                    'outer: loop {
                        println!("\nEnter the path to the TLS key file:");
                        let p = read_line_stdin().await?;
                        if !fs::try_exists(&p).await? {
                            eprintln!(
                                "{} '{}' or provide '{}' value?",
                                "The file does not exist.".red(),
                                "ignore".green(),
                                "new".green()
                            );
                            loop {
                                let v = read_line_stdin().await?.trim().to_lowercase();
                                if v == "ignore" {
                                    break;
                                } else if v != "new" {
                                    continue 'outer;
                                } else {
                                    eprintln!("{}", "Invalid value, please try again.".red());
                                }
                            }
                        }
                        slf.tls_key_path = Some(p);
                        break;
                    }

                    'outer: loop {
                        println!("Enter the path to the TLS cert file:");
                        let p = read_line_stdin().await?;
                        if !fs::try_exists(&p).await? {
                            eprintln!(
                                "{} '{}' or provide '{}' value?",
                                "The file does not exist.".red(),
                                "ignore".green(),
                                "new".green()
                            );
                            loop {
                                let v = read_line_stdin().await?.trim().to_lowercase();
                                if v == "ignore" {
                                    break;
                                } else if v != "new" {
                                    continue 'outer;
                                } else {
                                    eprintln!("{}", "Invalid value, please try again.".red());
                                }
                            }
                        }
                        slf.tls_cert_path = Some(p);
                        break;
                    }
                }
            }
        }

        println!("\nWill Rauthy run behind a reverse proxy?");
        slf.proxy_mode = read_line_stdin_yes().await?;
        if slf.proxy_mode {
            println!(
                r#"
Provide a list of trusted proxy CIDRs. When `proxy_mode = true`
or `peer_ip_header_name` is set, these are mandatory to
be able to extract the real client IP properly and safely
to prevent IP header spoofing. All requests with a
different source will be blocked."#
            );

            'outer: loop {
                println!("IP address range (e.g. 192.168.0.0/24): ");
                match read_line_stdin().await?.trim().parse::<IpCidr>() {
                    Ok(ip) => {
                        slf.trusted_proxies.push(ip);

                        println!("'{}' another one or '{}'?", "add".green(), "finish".green());
                        loop {
                            let v = read_line_stdin().await?.trim().to_lowercase();
                            if v == "add" {
                                continue 'outer;
                            } else if v == "finish" {
                                break 'outer;
                            } else {
                                eprintln!("{}", "Invalid value, please try again.".red());
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("{}", "Invalid IP address format. Please try again.".red());
                    }
                }
            }
        }

        ///////////////////////
        // Webauthn
        ///////////////////////
        println!(
            r#"
Provide the Public URL of the whole deployment
The LISTEN_SCHEME + PUB_URL must match the HTTP ORIGIN HEADER
later on, which is especially important when running Rauthy
behind a reverse proxy. In case of a non-standard port (80/443),
you need to add the port to the PUB_URL."#
        );
        slf.pub_url = read_line_stdin().await?;
        if slf.pub_url.contains("://") {
            slf.pub_url = slf.pub_url.split_once("://").unwrap().0.to_string();
        }

        println!(
            r#"
Provide the 'Relaying Party (RP) ID' for Webauthn (Passkeys).
This is the effective domain name.
E.g. if your PUB_URL is 'iam.example.com', you would use the
same as RP ID.
CAUTION: When this changes, already registered devices will
stop working and users cannot log in anymore! This is a
set-once-and-never-change value!"#
        );
        slf.rp_id = read_line_stdin().await?;

        println!(
            r#"
Provide the 'Relaying Party (RP)' Origin for Webauthn (Passkeys).
This is basically the RP ID from above, but with the HTTPS scheme
AND ALWAYS the port! Even if it's a default port!
E.g. if your PUB_URL / RP_ID is 'iam.example.com', you would use
'https://iam.example.com:443'."#
        );
        slf.rp_origin = read_line_stdin().await?;

        println!(
            r#"
Your 'Relaying Party (RP)' Name for Webauthn (Passkeys).
This is a non-critical value that could be changed at any time.
Some browsers will display this name during the Webauthn Ceremony.
It has just informative purposes."#
        );
        slf.rp_name = read_line_stdin().await?;

        ///////////////////////
        // E-Mail
        ///////////////////////
        println!(
            "\nWe need to configure an SMTP server for things like password resets \
        and other important events."
        );
        println!("SMTP URL:");
        slf.smtp_url = read_line_stdin().await?;
        println!("SMTP Username:");
        slf.smtp_username = read_line_stdin().await?;
        slf.smtp_password = PromptPassword::default()
            .prompt("SMTP Password: ".to_string())
            .await?;
        println!("SMTP 'From' in the format 'Some Admin <some.admin@localhost>'");
        slf.smtp_from = read_line_stdin().await?;

        ///////////////////////
        // Addons
        ///////////////////////
        println!("\nForce MFA for Rauthy Admins?");
        slf.admin_force_mfa = read_line_stdin_yes().await?;

        println!("\nEnable open user registration? By default, an admin muster add new users.");
        slf.user_registration_enable = read_line_stdin_yes().await?.then_some(true);

        println!("\nShould users be able to delete themselves?");
        slf.user_self_delete_enable = read_line_stdin_yes().await?.then_some(true);

        Ok(slf)
    }
}

pub async fn generate(args: ArgsGenConfig) -> Result<(), StdError> {
    if fs::try_exists(&args.output_file).await?
        && !args.overwrite
        && !fs::read(&args.output_file).await?.is_empty()
    {
        return Err(format!(
            "Cannot generate '{}' - file exists already",
            args.output_file
        )
        .into());
    }

    let config = GenConfig::prompt().await?;

    let _ = fs::remove_file(&args.output_file).await;
    let path = PathBuf::from(&args.output_file);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(&path, config.to_string()).await?;

    #[cfg(target_family = "unix")]
    set_access_rights(path).await?;

    println!("\n\n{}\n", "=".repeat(40));

    println!("Configuration written to: {}", args.output_file);
    println!(
        r#"
Keep in mind that this is only to get you started. This is by no means
a full config. You can configure a lot more things, and especially when
finally going into production, you should check the Book:
https://sebadob.github.io/rauthy
and / or the reference config with all available options:
https://sebadob.github.io/rauthy/config/config.html

NOTE: Depending on how you generated this file, you may need to adjust
the uid:gid. If you plan to use it inside the original container directly,
the file needs to be owned by user 10001:

chown 10001:10001 /path/to/file
"#
    );

    Ok(())
}

async fn set_access_rights(path: PathBuf) -> Result<(), StdError> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(&path, Permissions::from_mode(0o600)).await?;

    Ok(())
}

#[derive(Validate)]
struct ValidateEmail<'a> {
    #[validate(email)]
    email: &'a str,
}
