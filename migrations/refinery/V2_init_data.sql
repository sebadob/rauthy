insert into clients (id, name, enabled, confidential, secret, secret_kid, redirect_uris, post_logout_redirect_uris,
                     allowed_origins, flows_enabled, access_token_alg, id_token_alg, auth_code_lifetime,
                     access_token_lifetime, scopes, default_scopes, challenge, force_mfa, client_uri, contacts)
values ('rauthy', 'Rauthy', true, false, null, null,
        'http://localhost:8080/auth/v1/oidc/*,http://localhost:5173/auth/v1/oidc/*',
        'http://localhost:8080/auth/v1/*,http://localhost:5173/auth/v1/*', 'http://localhost:5173',
        'authorization_code,password', 'EdDSA', 'EdDSA', 60, 1800, 'openid,email,profile,groups', 'openid', 'S256',
        false, null, null),
       ('init_client', 'Init Client', true, true,
        E'\\x0101001600006256437954734761676756793579715147746CA4685D12B041464FEA154EAF7BEED2C71475DA6C3AE070688F9ABECD87539D54C9E981A5C02BAC2E9AEAFB5C0DF8D516218F4CD677CF180AF42FC4354F34DCBBC666784A9EBDF3524B8AE80D669575B1780CFB667CA19F4F1F',
        'bVCyTsGaggVy5yqQ', 'http://localhost:3000/oidc/callback', 'http://localhost:8080', 'http://localhost:8080/*',
        'authorization_code,password,client_credentials,refresh_token', 'EdDSA', 'RS512', 60, 60,
        'openid,email,profile,groups', 'openid', 'S256,plain', false, null, null);

insert into config (id, data)
values ('password_policy', E'\\x0E000000800000000101000000010100000001010000000001B40000000103000000');

insert into groups (id, name)
values ('GfQ7Eghqnmc2qLWu5TF25vus', 'user'),
       ('vjYA59RaZ5Kgqzch5VJVWmyo', 'admin');

insert into roles (id, name)
values ('3kv6Yokr3qsgepwQWyXJWLzs', 'admin'),
       ('9Pr9NVgu4HGLH7SnMB4a73i8', 'user'),
       ('JJsvMJH7ZJXC3xRbocw8E7Um', 'rauthy_admin');

insert into public.scopes (id, name, attr_include_access, attr_include_id)
values ('26D4MoZBSUTZb2pnDP9r24tn', 'email', null, null),
       ('XpQDpteCrvByhXYcnf9n2gyN', 'openid', null, null),
       ('YKZvMGRaE6zEdgk4jabQNfDY', 'profile', null, null),
       ('fwL9emEMcnbHyNE8n3pUs9J5', 'groups', null, null),
       ('awyOUa5rjFkowidSw03otjXU', 'address', null, null),
       ('ImGlzlvfeOrJ9iY394AVeJgJ', 'phone', null, null);

insert into public.users (id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
                          password_expires, created_at, last_login, last_failed_login, failed_login_attempts, language,
                          webauthn_user_id, user_expires, auth_provider_id, federation_uid, picture_id)
values ('2PYV3STNz3MN7VnPjJVcPQap', 'test_admin@localhost.de', 'Admin', 'Test', null, 'rauthy_admin', 'admin', true,
        true, null, 1680971459, null, null, null, 'en', null, null, null, null, null),
       ('m4PJ3TnyP32LA8hzY23deme3', 'init_admin@localhost.de', 'Admin', 'Init',
        '$argon2id$v=19$m=32768,t=3,p=2$PoAwNZm4cqhNK0hQV2hSOQ$Xc2ECqWOU3aVvVLswmu29Hk7eGTYmKQQgo2qtyp2GwQ',
        'rauthy_admin', 'admin', true, true, null, 1680971459, null, null, null, 'en', null, null, null, null, null),
       ('za9UxpH7XVxqrtpEbThoqvn2', 'admin@localhost.de', 'Rauthy', 'Admin',
        '$argon2id$v=19$m=32768,t=3,p=2$xr23OhOHw+pNyy3dYKZUcA$CBO4NpGvyi6gvrb5uNrnsS/z/Ew+IuS0/gVqFmLKncA',
        'rauthy_admin,admin', 'admin', true, true, null, 1680971457, 1680975378, null, null, 'en', null, null, null,
        null, null);