# Lookup Table ENV var -> TOML value

| ENV VAR                                    | TOML path                                   | type       | required |
|--------------------------------------------|---------------------------------------------|------------|----------|
| USERINFO_STRICT                            | access.userinfo_strict                      | bool       |          |
| DANGER_DISABLE_INTROSPECT_AUTH             | access.danger_disable_introspect_auth       | bool       |          |
| DISABLE_REFRESH_TOKEN_NBF                  | access.disable_refresh_token_nbf            | bool       |          |
| SEC_HEADER_BLOCK                           | access.sec_header_block                     | bool       |          |
| SESSION_VALIDATE_IP                        | access.session_validate_ip                  | bool       |          |
| PASSWORD_RESET_COOKIE_BINDING              | access.password_reset_cookie_binding        | bool       |          |
| PEER_IP_HEADER_NAME                        | access.peer_ip_header_name                  | String     |          |
| COOKIE_MODE                                | access.cookie_mode                          | String     |          |
| COOKIE_SET_PATH                            | access.cookie_set_path                      | bool       |          |
| TOKEN_LEN_LIMIT                            | access.token_len_limit                      | u32        |          |
| AUTH_HEADERS_ENABLE                        | auth_headers.enable                         | bool       |          |
| AUTH_HEADER_USER                           | auth_headers.user                           | String     |          |
| AUTH_HEADER_ROLES                          | auth_headers.roles                          | String     |          |
| AUTH_HEADER_GROUPS                         | auth_headers.groups                         | String     |          |
| AUTH_HEADER_EMAIL                          | auth_headers.email                          | String     |          |
| AUTH_HEADER_EMAIL_VERIFIED                 | auth_headers.email_verified                 | String     |          |
| AUTH_HEADER_FAMILY_NAME                    | auth_headers.family_name                    | String     |          |
| AUTH_HEADER_GIVEN_NAME                     | auth_headers.given_name                     | String     |          |
| AUTH_HEADER_MFA                            | auth_headers.mfa                            | String     |          |
| BACKCHANNEL_LOGOUT_RETRY_COUNT             | backchannel_logout.retry_count              | u16        |          |
| BACKCHANNEL_DANGER_ALLOW_HTTP              | REMOVED -> global http_client used now      |            |          |
| BACKCHANNEL_DANGER_ALLOW_INSECURE          | REMOVED -> global http_client used now      |            |          |
| LOGOUT_TOKEN_LIFETIME                      | backchannel_logout.token_lifetime           | u32        |          |
| LOGOUT_TOKEN_ALLOW_CLOCK_SKEW              | backchannel_logout.allow_clock_skew         | u32        |          |
| LOGOUT_TOKEN_ALLOWED_LIFETIME              | backchannel_logout.allowed_token_lifetime   | u32        |          |
| BOOTSTRAP_ADMIN_EMAIL                      | bootstrap.admin_email                       | String     |          |
| BOOTSTRAP_ADMIN_PASSWORD_PLAIN             | bootstrap.password_plain                    | String     |          |
| BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID          | bootstrap.pasword_argon2id                  | String     |          |
| BOOTSTRAP_API_KEY                          | bootstrap.api_key                           | String     |          |
| BOOTSTRAP_API_KEY_SECRET                   | bootstrap.api_key_secret                    | String     |          |
| HQL_NODE_ID_FROM                           | cluster.node_id_from                        | "k8s"      | x *1     |
| HQL_NODE_ID                                | cluster.node_id                             | u64        | x *1     |
| HQL_NODES                                  | cluster.nodes                               | \[String\] | x        |
| HQL_DATA_DIR                               | cluster.data_dir                            | String     |          |
| HQL_FILENAME_DB                            | cluster.filename_db                         | String     |          |
| HQL_LOG_STATEMENTS                         | cluster.log_statements                      | bool       |          |
| -                                          | cluster.prepared_statement_cache_capacity   | u16        |          |
| HQL_READ_POOL_SIZE                         | cluster.read_pool_size                      | u16        |          |
| HQL_LOG_SYNC                               | cluster.log_sync                            | String     |          |
| HQL_WAL_SIZE                               | cluster.wal_size                            | u32        |          |
| HQL_CACHE_STORAGE_DISK                     | cluster.cache_storage_disk                  | bool       |          |
| HQL_LOGS_UNTIL_SNAPSHOT                    | cluster.logs_until_snapshot                 | u64        |          |
| HQL_SHUTDOWN_DELAY_MILLS                   | cluster.shutdown_delay_millis               | u32        |          |
| HQL_TLS_RAFT_KEY                           | cluster.tls_raft_key                        | String     |          |
| HQL_TLS_RAFT_CERT                          | cluster.tls_raft_cert                       | String     |          |
| -                                          | cluster.tls_raft_danger_tls_no_verify       | bool       |          |
| HQL_TLS_API_KEY                            | cluster.tls_api_key                         | String     |          |
| HQL_TLS_RAFT_KEY                           | cluster.tls_api_cert                        | String     |          |
| -                                          | cluster.tls_api_danger_tls_no_verify        | bool       |          |
| HQL_SECRET_RAFT                            | cluster.secret_raft                         | String     | x        |
| HQL_SECRET_API                             | cluster.secret_api                          | String     | x        |
| -                                          | cluster.health_check_delay_secs             | u32        |          |
| HQL_BACKUP_CRON                            | cluster.backup_cron                         | String     |          |
| HQL_BACKUP_KEEP_DAYS                       | cluster.backup_keep_days                    | u16        |          |
| HQL_BACKUP_KEEP_DAYS_LOCAL                 | s3_url.backup_keep_days_local               | u16        |          |
| HQL_BACKUP_RESTORE                         | -                                           | String     |          |
| HQL_BACKUP_SKIP_VALIDATION                 | -                                           | bool       |          |
| HQL_S3_URL                                 | cluster.s3_url                              | String     | *2       |
| HQL_S3_BUCKET                              | cluster.s3_bucket                           | String     | *2       |
| HQL_S3_REGION                              | cluster.s3_bucket                           | String     | *2       |
| HQL_S3_PATH_STYLE                          | cluster.s3_path_style                       | bool       |          |
| HQL_S3_KEY                                 | cluster.s3_key                              | String     | *2       |
| HQL_S3_SECRET                              | cluster.s3_secret                           | String     | *2       |
| HQL_PASSWORD_DASHBOARD                     | cluster.password_dashboard                  | String     |          |
| HQL_INSECURE_COOKIE                        | cluster.insecure_cookie                     | bool       |          |
| HQL_WAL_IGNORE_LOCK                        | cluster.wal_ignore_lock                     | bool       |          |
| HQL_DANGER_RAFT_STATE_RESET                | -                                           | bool       |          |
| HIQLITE                                    | database.hiqlite                            | bool       |          |
| HEALTH_CHECK_DELAY_SECS                    | database.health_check_delay_secs            | u32        |          |
| PG_HOST                                    | database.pg_host                            | String     | *3       |
| PG_PORT                                    | database.pg_port                            | u16        |          |
| PG_USER                                    | database.pg_user                            | String     | *3       |
| PG_PASSWORD                                | database.pg_password                        | String     | *3       |
| PG_DB_NAME                                 | database.pg_db_name                         | String     |          |
| PG_TLS_NO_VERIFY                           | database.pg_tls_no_verify                   | bool       |          |
| PG_MAX_CONN                                | database.pg_max_conn                        | u16        |          |
| MIGRATE_DB_FROM                            | -                                           | String     |          |
| MIGRATE_PG_HOST                            | database.migrate_pg_host                    | String     | *4       |
| MIGRATE_PG_PORT                            | database.migrate_pg_port                    | u16        |          |
| MIGRATE_PG_USER                            | database.migrate_pg_user                    | String     | *4       |
| MIGRATE_PG_PASSWORD                        | database.migrate_pg_password                | String     | *4       |
| MIGRATE_PG_DB_NAME                         | database.migrate_pg_db_name                 | String     |          |
| SCHED_USER_EXP_MINS                        | database.sched_user_exp_mins                | u32        |          |
| SCHED_USER_EXP_DELETE_MINS                 | database.sched_user_exp_delete_mins         | u32        |          |
| DEVICE_GRANT_CODE_LIFETIME                 | device_grant.code_lifetime                  | u32        |          |
| DEVICE_GRANT_USER_CODE_LENGTH              | device_grant.user_code_length               | u32        |          |
| DEVICE_GRANT_RATE_LIMIT                    | device_grant.rate_limit                     | u32        |          |
| DEVICE_GRANT_POLL_INTERVAL                 | device_grant.poll_interval                  | u32        |          |
| DEVICE_GRANT_REFRESH_TOKEN_LIFETIME        | device_grant.refresh_token_lifetime         | u32        |          |
| DPOP_FORCE_NONCE                           | dpop.force_nonce                            | bool       |          |
| DPOP_NONCE_EXP                             | dpop.nonce_exp                              | u32        |          |
| ENABLE_DYN_CLIENT_REG                      | dynamic_clients.enable                      | bool       |          |
| DYN_CLIENT_ALLOWED_SCOPES                  | dynamic_clients.allowed_scopes              | \[String\] |          |
| DYN_CLIENT_DEFAULT_SCOPES                  | dynamic_clients.default_scopes              | \[String\] |          |
| DYN_CLIENT_REG_TOKEN                       | dynamic_clients.reg_token                   | String     | *5       |
| DYN_CLIENT_DEFAULT_TOKEN_LIFETIME          | dynamic_clients.default_token_lifetime      | u32        |          |
| DYN_CLIENT_SECRET_AUTO_ROTATE              | dynamic_clients.secret_auto_rotate          | bool       |          |
| DYN_CLIENT_CLEANUP_INTERVAL                | dynamic_clients.cleanup_interval            | u32        |          |
| DYN_CLIENT_CLEANUP_MINUTES                 | dynamic_clients.cleanup_minutes             | u32        |          |
| DYN_CLIENT_CLEANUP_INACTIVE_DAYS           | dynamic_clients.cleanup_inactive_days       | u32        |          |
| DYN_CLIENT_RATE_LIMIT_SEC                  | dynamic_clients.rate_limit_sec              | u32        |          |
| RAUTHY_ADMIN_EMAIL                         | email.rauthy_admin_email                    | String     |          |
| EMAIL_SUB_PREFIX                           | email.sub_prefix                            | String     |          |
| SMTP_URL                                   | email.smtp_url                              | String     | *6       |
| SMTP_PORT                                  | email.smtp_port                             | u16        |          |
| SMTP_USERNAME                              | email.smtp_username                         | String     |          |
| SMTP_PASSWORD                              | email.smtp_password                         | String     |          |
| SMTP_FROM                                  | email.smtp_from                             | String     |          |
| SMTP_CONNECT_RETRIES                       | email.connect_retries                       | u16        |          |
| SMTP_DANGER_INSECURE                       | email.danger_insecure                       | bool       |          |
| ENC_KEYS                                   | encryption.keys                             | \[String\] | x        |
| ENC_KEY_ACTIVE                             | encryption.key_active                       | String     | x        |
| ENABLE_EPHEMERAL_CLIENTS                   | ephemeral_clients.enable                    | bool       |          |
| ENABLE_WEB_ID                              | ephemeral_clients.enable_web_id             | bool       |          |
| ENABLE_SOLID_AUD                           | ephemeral_clients.enable_solid_aud          | bool       |          |
| EPHEMERAL_CLIENTS_FORCE_MFA                | ephemeral_clients.force_mfa                 | bool       |          |
| EPHEMERAL_CLIENTS_ALLOWED_FLOWS            | ephemeral_clients.allowed_flows             | \[String\] |          |
| EPHEMERAL_CLIENTS_ALLOWED_SCOPES           | ephemeral_clients.allowed_scopes            | \[String\] |          |
| EPHEMERAL_CLIENTS_CACHE_LIFETIME           | ephemeral_clients.cache_lifetime            | u32        |          |
| EVENT_EMAIL                                | events.email                                | String     |          |
| EVENT_MATRIX_USER_ID                       | events.matrix_user_id                       | String     |          |
| EVENT_MATRIX_ROOM_ID                       | events.matrix_room_id                       | String     |          |
| EVENT_MATRIX_ACCESS_TOKEN                  | events.matrix_access_token                  | String     |          |
| EVENT_MATRIX_USER_PASSWORD                 | events.matrix_user_password                 | String     |          |
| EVENT_MATRIX_SERVER_URL                    | events.matrix_server_url                    | String     |          |
| EVENT_MATRIX_ROOT_CA_PATH                  | events.matrix_root_ca_path                  | String     |          |
| EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION | events.matrix_danger_disable_tls_validation | bool       |          |
| EVENT_MATRIX_ERROR_NO_PANIC                | events.matrix_error_no_panic                | bool       |          |
| EVENT_SLACK_WEBHOOK                        | events.slack_webhook                        | String     |          |
| EVENT_NOTIFY_LEVEL_EMAIL                   | events.notify_level_email                   | Level      |          |
| EVENT_NOTIFY_LEVEL_MATRIX                  | events.notify_level_matrix                  | Level      |          |
| EVENT_NOTIFY_LEVEL_SLACK                   | events.notify_level_slack                   | Level      |          |
| EVENT_PERSIST_LEVEL                        | events.persist_level                        | Level      |          |
| EVENT_CLEANUP_DAYS                         | events.cleanup_days                         | u32        |          |
| EVENT_LEVEL_NEW_USER                       | events.level_new_user                       | Level      |          |
| EVENT_LEVEL_USER_EMAIL_CHANGE              | events.level_user_email_change              | Level      |          |
| EVENT_LEVEL_USER_PASSWORD_RESET            | events.level_user_password_reset            | Level      |          |
| EVENT_LEVEL_RAUTHY_ADMIN                   | events.level_rauthy_admin                   | Level      |          |
| EVENT_LEVEL_RAUTHY_VERSION                 | events.level_rauthy_version                 | Level      |          |
| EVENT_LEVEL_JWKS_ROTATE                    | events.level_jwks_rotate                    | Level      |          |
| EVENT_LEVEL_SECRETS_MIGRATED               | events.level_secrets_migrated               | Level      |          |
| EVENT_LEVEL_RAUTHY_START                   | events.level_rauthy_start                   | Level      |          |
| EVENT_LEVEL_RAUTHY_HEALTHY                 | events.level_rauthy_healthy                 | Level      |          |
| EVENT_LEVEL_RAUTHY_UNHEALTHY               | events.level_rauthy_unhealthy               | Level      |          |
| EVENT_LEVEL_IP_BLACKLISTED                 | events.level_ip_blacklisted                 | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_25               | events.level_failed_logins_25               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_20               | events.level_failed_logins_20               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_15               | events.level_failed_logins_15               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_10               | events.level_failed_logins_10               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_7                | events.level_failed_logins_7                | Level      |          |
| EVENT_LEVEL_FAILED_LOGIN                   | events.level_failed_login                   | Level      |          |
| DISABLE_APP_VERSION_CHECK                  | events.disable_app_version_check            | bool       |          |
| EXPERIMENTAL_FED_CM_ENABLE                 | fedcm.experimental_enable                   | bool       |          |
| SESSION_LIFETIME_FED_CM                    | fedcm.session_lifetime                      | u32        |          |
| SESSION_TIMEOUT_FED_CM                     | fedcm.session_timeout                       | u32        |          |
| ARGON2_M_COST                              | hashing.argon2_m_cost                       | u32        |          |
| ARGON2_T_COST                              | hashing.argon2_t_cost                       | u32        |          |
| ARGON2_P_COST                              | hashing.argon2_p_cost                       | u32        |          |
| MAX_HASH_THREADS                           | hashing.max_hash_threads                    | u32        |          |
| HASH_AWAIT_WARN_TIME                       | hashing.hash_await_warn_time                | u32        |          |
| JWK_AUTOROTATE_CRON                        | hashing.jwk_autorotate_cron                 | String     |          |
| HTTP_CONNECT_TIMEOUT                       | http_client.connect_timeout                 | u32        |          |
| HTTP_REQUEST_TIMEOUT                       | http_client.request_timeout                 | u32        |          |
| HTTP_MIN_TLS                               | http_client.min_tls                         | String     |          |
| HTTP_IDLE_TIMEOUT                          | http_client.idle_timeout                    | u32        |          |
| HTTP_DANGER_UNENCRYPTED                    | http_client.danger_unencrypted              | bool       |          |
| HTTP_DANGER_INSECURE                       | http_client.danger_insecure                 | bool       |          |
| HTTP_CUST_ROOT_CA_BUNDLE                   | http_client.root_ca_bundle                  | String     |          |
| FILTER_LANG_COMMON                         | i18n.filter_lang_common                     | \[String\] |          |
| FILTER_LANG_ADMIN                          | i18n.filter_lang_admin                      | \[String\] |          |
| REFRESH_TOKEN_GRACE_TIME                   | lifetimes.refresh_token_grace_time          | u16        |          |
| REFRESH_TOKEN_LIFETIME                     | lifetimes.refresh_token_lifetime            | u16        |          |
| SESSION_LIFETIME                           | lifetimes.session_lifetime                  | u32        |          |
| SESSION_RENEW_MFA                          | lifetimes.session_renew_mfa                 | bool       |          |
| SESSION_TIMEOUT                            | lifetimes.session_timeout                   | u32        |          |
| ML_LT_PWD_RESET                            | lifetimes.magic_link_pwd_reset              | u32        |          |
| ML_LT_PWD_FIRST                            | lifetimes.magic_link_pwd_first              | u32        |          |
| LOG_LEVEL                                  | logging.level                               | Level      |          |
| LOG_LEVEL_DATABASE                         | logging.level_database                      | Level      |          |
| LOG_LEVEL_ACCESS                           | logging.level_access                        | String     |          |
| LOG_FMT                                    | logging.log_fmt                             | "json"     |          |
| ADMIN_FORCE_MFA                            | mfa.admin_force_mfa                         | bool       |          |
| POW_DIFFICULTY                             | pow.difficulty                              | u16        |          |
| POW_EXP                                    | pow.exp                                     | u16        |          |
| SCIM_SYNC_DELETE_GROUPS                    | scim.sync_delete_groups                     | bool       |          |
| SCIM_SYNC_DELETE_USERS                     | scim.sync_delete_users                      | bool       |          |
| SCIM_RETRY_COUNT                           | scim.retry_count                            | u16        |          |
| LISTEN_ADDRESS                             | server.listen_address                       | String     |          |
| LISTEN_PORT_HTTP                           | server.port_http                            | u16        |          |
| LISTEN_PORT_HTTPS                          | server.port_https                           | u16        |          |
| LISTEN_SCHEME                              | server.scheme                               | String     |          |
| PUB_URL                                    | server.pub_url                              | String     | x        |
| HTTP_WORKERS                               | server.http_workers                         | u16        |          |
| PROXY_MODE                                 | server.proxy_mode                           | bool       | *7       |
| TRUSTED_PROXIES                            | server.trusted_proxies                      | \[String\] | *7       |
| ADDITIONAL_ALLOWED_ORIGIN_SCHEMES          | server.additional_allowed_origin_schemes    | \[String\] |          |
| METRICS_ENABLE                             | server.metrics_enable                       | bool       |          |
| METRICS_ADDR                               | server.metrics_addr                         | String     |          |
| METRICS_PORT                               | server.metrics_port                         | u16        |          |
| SWAGGER_UI_ENABLE                          | server.swagger_ui_enable                    | bool       |          |
| SWAGGER_UI_PUBLIC                          | server.swagger_ui_public                    | bool       |          |
| SSE_KEEP_ALIVE                             | server.see_keep_alive                       | u16        |          |
| SSP_THRESHOLD                              | server.ssp_threshold                        | u16        |          |
| SUSPICIOUS_REQUESTS_BLACKLIST              | suspicious_requests.blacklist               | u16        |          |
| SUSPICIOUS_REQUESTS_LOG                    | suspicious_requests.log                     | bool       |          |
| -                                          | \[templates\].lang                          | String     | *8       |
| -                                          | \[templates\].typ                           | String     | *8       |
| -                                          | \[templates\].subject                       | String     |          |
| -                                          | \[templates\].header                        | String     |          |
| -                                          | \[templates\].text                          | String     |          |
| -                                          | \[templates\].click_link                    | String     |          |
| -                                          | \[templates\].validity                      | String     |          |
| -                                          | \[templates\].expires                       | String     |          |
| -                                          | \[templates\].footer                        | String     |          |
| TLS_CERT                                   | tls.cert_path                               | String     |          |
| TLS_KEY                                    | tls.key_path                                | String     |          |
| PICTURE_STORAGE_TYPE                       | user_pictures.storage_type                  | String     |          |
| PICTURE_PATH                               | user_pictures.path                          | String     |          |
| PIC_S3_URL                                 | user_pictures.s3_url                        | String     | *2       |
| PIC_S3_BUCKET                              | user_pictures.bucket                        | String     | *2       |
| PIC_S3_REGION                              | user_pictures.region                        | String     | *2       |
| PIC_S3_KEY                                 | user_pictures.s3_key                        | String     | *2       |
| PIC_S3_SECRET                              | user_pictures.s3_secret                     | String     | *2       |
| PIC_S3_PATH_STYLE                          | user_pictures.s3_path_style                 | bool       |          |
| PICTURE_UPLOAD_LIMIT_MB                    | user_pictures.upload_limit_mb               | u16        |          |
| PICTURE_PUBLIC                             | user_pictures.public                        | bool       |          |
| OPEN_USER_REG                              | user_registration.enable                    | bool       |          |
| USER_REG_DOMAIN_RESTRICTION                | user_registration.domain_restriction        | String     |          |
| USER_REG_DOMAIN_BLACKLIST                  | user_registration.domain_blacklist          | \[String\] |          |
| USER_REG_OPEN_REDIRECT                     | user_registration.allow_open_redirect       | bool       |          |
| RP_ID                                      | webauthn.rp_id                              | String     | x        |
| RP_ORIGIN                                  | webauthn.rp_origin                          | String     | x        |
| RP_NAME                                    | webauthn.rp_name                            | String     |          |
| WEBAUTHN_REQ_EXP                           | webauthn.req_exp                            | u16        |          |
| WEBAUTHN_DATA_EXP                          | webauthn.data_exp                           | u16        |          |
| WEBAUTHN_RENEW_EXP                         | webauthn.renew_exp                          | u16        |          |
| WEBAUTHN_FORCE_UV                          | webauthn.force_uv                           | bool       |          |
| WEBAUTHN_NO_PASSWORD_EXPIRY                | webauthn.no_password_exp                    | bool       |          |

1. At least one of `cluster.node_id_from` / `cluster.node_id` is required
2. When `s3_url` is given, the other `s3_*` values are expected as well
3. Required when `database.hiqlite = false`
4. Required when `MIGRATE_DB_FROM=postgres`
5. Not strictly required but should probably almost be set when `dynamic_clients.enable = true` to not have an open dyn
   client registration.
6. When not set, E-Mail cannot be sent and things like user registration and self-service password requests will not
   work. You can operate Rauthy without this setting, but then an Admin needs to perform all these actions.
7. Required when running behind a reverse proxy
8. The `[templates]` block can be given multiple times for different languages / templates, but if so, `lang` + `typ`
   are required inside.

> NOTE: All `\[String\]` types are Arrays inside the TOML, but a single String value for an ENV VAR, which separates the
> values by `\n`.

> All `Level` values can be one of: 'info', 'notice', 'warning', 'critical'

> Quite a few of these values, even when they are a `String` type, expect a certain format. Take a look at the reference
> config for more information on each one.
