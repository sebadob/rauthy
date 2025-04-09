alter table clients
    add force_mfa bool default false not null;

alter table clients
    alter column force_mfa drop default;
