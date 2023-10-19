create table api_keys
(
    name             varchar           not null
        constraint api_keys_pk
            primary key,
    secret           blob              not null,
    is_hash_uptodate bool default true not null,
    created          bigint            not null,
    expires          bigint,
    enc_key_id       varchar           not null,
    access           blob              not null
);

create index api_keys_enc_key_id_index
    on api_keys (enc_key_id);

create index api_keys_expires_index
    on api_keys (expires);
