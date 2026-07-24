create table fido_mds_certs
(
    hash bytea not null
        constraint fido_mds_certs_pk
            primary key,
    der  bytea not null
);

create table fido_mds_entries
(
    aaguid            bytea    not null
        constraint fido_mds_entries_pk
            primary key,
    description       varchar  not null,
    key_protection    bigint   not null,
    attachment_hint   bigint   not null,
    attestation_types bigint   not null,
    cert_level        smallint not null
);

create table fido_mds_entry_certs
(
    aaguid    bytea not null
        constraint fido_mds_entry_certs_entry_fk
            references fido_mds_entries
            on update cascade on delete cascade,
    cert_hash bytea not null
        constraint fido_mds_entry_certs_cert_fk
            references fido_mds_certs (hash)
            on update cascade on delete cascade,
    constraint fido_mds_entry_certs_pk
        primary key (aaguid, cert_hash)
);

alter table passkeys
    add aaguid bytea
        constraint passkeys_fido_mds_entries_fk
            references fido_mds_entries
            on update cascade on delete set null;
