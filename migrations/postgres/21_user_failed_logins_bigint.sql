-- alter the failed_login_attempts to a bigint to get back compile time checked queries
alter table users
    alter column failed_login_attempts type bigint
        using failed_login_attempts::bigint;
