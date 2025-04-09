alter table refresh_tokens
    add is_mfa bool default false not null;
