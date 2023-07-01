insert into clients (id, name, enabled, confidential, secret, secret_kid, redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg, id_token_alg, refresh_token, auth_code_lifetime, access_token_lifetime, scopes, default_scopes, challenge)
values  ('init_client', 'Init Client', 1, 1, x'45AEEA59D2E65A9178FF90AEDC1CB7F3C8C970912A99594A8317FE0CE7223E2150B261C35DED44410110F724852AAD7A7C792957C82A42013B3F1F06B79894F9B6BFE5E47AAD5E3EB46CE52F8A1EFAE89F64AC6636DF60547F245A24', 'bVCyTsGaggVy5yqQ', 'http://localhost:3000/oidc/callback', 'http://localhost:8080', 'http://localhost:8080/*', 'authorization_code,password,client_credentials,refresh_token', 'EdDSA', 'RS512', 1, 60, 60, 'openid,email,profile,groups', 'openid', 'S256,plain'),
        ('rauthy', 'Rauthy', 1, 0, null, null, 'http://localhost:8080/auth/v1/oidc/*,http://localhost:5173/auth/v1/oidc/*', 'http://localhost:8080/auth/v1/*,http://localhost:5173/auth/v1/*', 'http://localhost:5173', 'authorization_code,password', 'EdDSA', 'EdDSA', 0, 60, 1800, 'openid,email,profile,groups', 'openid', 'S256');

insert into config (id, data)
values  ('password_policy', x'0A000000800000000101000000010100000001010000000101000000015A0000000103000000');

insert into groups (id, name)
values  ('GfQ7Eghqnmc2qLWu5TF25vus', 'user'),
        ('vjYA59RaZ5Kgqzch5VJVWmyo', 'admin');

insert into roles (id, name)
values  ('3kv6Yokr3qsgepwQWyXJWLzs', 'admin'),
        ('9Pr9NVgu4HGLH7SnMB4a73i8', 'user'),
        ('JJsvMJH7ZJXC3xRbocw8E7Um', 'rauthy_admin');

insert into scopes (id, name, attr_include_access, attr_include_id)
values  ('26D4MoZBSUTZb2pnDP9r24tn', 'email', null, null),
        ('XpQDpteCrvByhXYcnf9n2gyN', 'openid', null, null),
        ('YKZvMGRaE6zEdgk4jabQNfDY', 'profile', null, null),
        ('fwL9emEMcnbHyNE8n3pUs9J5', 'groups', null, null);

insert into users (id, email, given_name, family_name, password, roles, groups, enabled, email_verified, password_expires, created_at, last_login, last_failed_login, failed_login_attempts, mfa_app, sec_key_1, sec_key_2)
values  ('2PYV3STNz3MN7VnPjJVcPQap', 'test_admin@localhost.de', 'Admin', 'Test', null, 'rauthy_admin', 'admin', 1, 1, null, 1680971459, null, null, null, null, null, null),
        ('m4PJ3TnyP32LA8hzY23deme3', 'init_admin@localhost.de', 'Admin', 'Init', '$argon2id$v=19$m=32768,t=3,p=2$PoAwNZm4cqhNK0hQV2hSOQ$Xc2ECqWOU3aVvVLswmu29Hk7eGTYmKQQgo2qtyp2GwQ', 'rauthy_admin', 'admin', 1, 1, null, 1680971459, null, null, null, null, null, null),
        ('za9UxpH7XVxqrtpEbThoqvn2', 'admin@localhost.de', 'Rauthy', 'Admin', '$argon2id$v=19$m=32768,t=3,p=2$xr23OhOHw+pNyy3dYKZUcA$CBO4NpGvyi6gvrb5uNrnsS/z/Ew+IuS0/gVqFmLKncA', 'rauthy_admin,admin', 'admin', 1, 1, null, 1680971457, 1680975378, null, null, null, null, null);
