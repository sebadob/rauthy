alter table auth_providers
    add auto_onboarding bool default true not null;

alter table auth_providers
    add auto_link bool default false not null;
