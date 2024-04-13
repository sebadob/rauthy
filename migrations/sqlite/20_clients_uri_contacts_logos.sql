-- modify the clients table and add 'force_mfa'

alter table clients
    rename to clients_old;

create table clients
(
    id                        varchar not null
        constraint clients_pk
            primary key,
    name                      varchar,
    enabled                   bool    not null,
    confidential              bool    not null,
    secret                    blob,
    secret_kid                varchar,
    redirect_uris             varchar not null,
    post_logout_redirect_uris varchar,
    allowed_origins           varchar,
    flows_enabled             varchar not null,
    access_token_alg          varchar not null,
    id_token_alg              varchar not null,
    refresh_token             bool    not null,
    auth_code_lifetime        integer not null,
    access_token_lifetime     integer not null,
    scopes                    varchar not null,
    default_scopes            varchar not null,
    challenge                 varchar,
    force_mfa                 bool    not null,
    client_uri                varchar,
    contacts                  varchar
);

insert into clients(id,
                    name,
                    enabled,
                    confidential,
                    secret,
                    secret_kid,
                    redirect_uris,
                    post_logout_redirect_uris,
                    allowed_origins,
                    flows_enabled,
                    access_token_alg,
                    id_token_alg,
                    refresh_token,
                    auth_code_lifetime,
                    access_token_lifetime,
                    scopes,
                    default_scopes,
                    challenge,
                    force_mfa)
select id,
       name,
       enabled,
       confidential,
       secret,
       secret_kid,
       redirect_uris,
       post_logout_redirect_uris,
       allowed_origins,
       flows_enabled,
       access_token_alg,
       id_token_alg,
       refresh_token,
       auth_code_lifetime,
       access_token_lifetime,
       scopes,
       default_scopes,
       challenge,
       false as force_mfa
from clients_old;

-- recreate all tables with foreign keys to clients

-- clients_dyn

alter table clients_dyn
    rename to clients_dyn_old;

drop index clients_dyn_last_used_index;

create table clients_dyn
(
    id                         text    not null
        constraint clients_dyn_pk
            primary key
        constraint clients_dyn_clients_id_fk
            references clients
            on update cascade on delete cascade,
    created                    integer not null,
    last_used                  integer,
    registration_token         blob    not null,
    token_endpoint_auth_method text    not null
);

create index clients_dyn_last_used_index
    on clients_dyn (last_used);

insert into clients_dyn(id, created, last_used, registration_token, token_endpoint_auth_method)
select id, created, last_used, registration_token, token_endpoint_auth_method
from clients_dyn_old;

-- colors
alter table colors
    rename to colors_old;

create table colors
(
    client_id varchar not null
        constraint colors_pk
            primary key
        constraint colors_clients_id_fk
            references clients
            on update cascade
            on delete cascade,
    data      blob    not null
);

insert into colors(client_id, data)
select client_id, data
from colors_old;

-- logos
alter table logos
    rename to logos_old;

create table logos
(
    client_id varchar not null
        constraint logos_pk
            primary key
        constraint logos_clients_id_fk
            references clients
            on update cascade
            on delete cascade,
    data      blob    not null
);

insert into logos(client_id, data)
select client_id, data
from logos_old;

-- finally, drop all the old tables

drop table clients_dyn_old;
drop table colors_old;
drop table logos_old;
drop table clients_old;

-- re-create the logos table

drop table logos;

create table client_logos
(
    client_id    varchar not null
        constraint client_logos_client_id_fk
            references clients
            on update cascade on delete cascade,
    res          varchar not null,
    content_type varchar not null,
    data         blob    not null,
    constraint client_logos_pk
        primary key (client_id, res)
);

-- rauthy default logo

insert or ignore into client_logos (client_id, res, content_type, data)
values ('rauthy', 'svg', 'image/svg+xml',
        '<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="100%" height="100%" viewBox="0 0 512 138" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
    <g transform="matrix(1,0,0,1,0,-11)">
        <g transform="matrix(1,0,0,1,0,-176)">
            <g transform="matrix(0.920325,0,0,1.84151,45.9279,26.459)">
                <rect x="27.741" y="151.57" width="200.517" height="10.148" style="fill:rgb(4,7,11);"/>
            </g>
            <g transform="matrix(1.93472,0,0,1.82732,8.35618,28.7533)">
                <rect x="33.307" y="97.15" width="94.693" height="54.42" style="fill:rgb(4,7,11);stroke:rgb(4,7,11);stroke-width:1.06px;"/>
            </g>
            <g transform="matrix(1.82732,0,0,1.82732,-160.822,70.1806)">
                <g transform="matrix(72,0,0,72,227.174,123.417)">
                </g>
                <text x="128.982px" y="123.417px" style="font-family:''Calibri-Bold'', ''Calibri'', sans-serif;font-weight:700;font-size:72px;fill:white;">r<tspan x="152.994px 188.537px " y="123.417px 123.417px ">au</tspan></text>
            </g>
            <g transform="matrix(1,0,0,1.01617,-1.42109e-14,-5.24492)">
                <path d="M440.936,322.643L439.204,324.266L255.482,324.266L255.482,305.721L440.936,305.721L440.936,322.643Z" style="fill:url(#_Linear1);"/>
            </g>
            <g transform="matrix(0.920191,0,0,1.84121,46.2464,-91.3383)">
                <rect x="27.741" y="151.57" width="200.517" height="10.148" style="fill:url(#_Linear2);"/>
            </g>
            <g transform="matrix(1.97598,0,0,1.84619,190.187,26.062)">
                <rect x="33.307" y="97.15" width="94.693" height="54.42" style="fill:rgb(43,65,107);"/>
            </g>
            <path d="M439.204,187.734L440.557,189.007L440.557,206.279L256,206.279L256,187.734L439.204,187.734Z" style="fill:rgb(43,65,107);"/>
            <g transform="matrix(1.82732,0,0,1.82732,-154.661,70.1806)">
                <g transform="matrix(72,0,0,72,323.045,123.417)">
                </g>
                <text x="226.646px" y="123.417px" style="font-family:''Calibri-Bold'', ''Calibri'', sans-serif;font-weight:700;font-size:72px;fill:white;">th<tspan x="288.943px " y="123.417px ">y</tspan></text>
            </g>
            <g transform="matrix(2,0,0,2,0,0)">
                <path d="M219.602,93.867L256,128L219.602,162.133L219.602,93.867Z" style="fill:rgb(43,65,107);"/>
            </g>
            <g transform="matrix(2,0,0,1.95739,0,3.99997)">
                <path d="M36.398,93.867L0,93.867L35.908,128.524L0,163.619L36.398,163.619" style="fill:rgb(4,7,11);"/>
            </g>
        </g>
    </g>
    <defs>
        <linearGradient id="_Linear1" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(185.454,0,0,18.5443,255.482,314.994)"><stop offset="0" style="stop-color:rgb(4,7,11);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(43,65,107);stop-opacity:1"/></linearGradient>
        <linearGradient id="_Linear2" x1="0" y1="0" x2="1" y2="0" gradientUnits="userSpaceOnUse" gradientTransform="matrix(200.517,0,0,10.1483,27.7414,156.645)"><stop offset="0" style="stop-color:rgb(4,7,11);stop-opacity:1"/><stop offset="1" style="stop-color:rgb(43,65,107);stop-opacity:1"/></linearGradient>
    </defs>
</svg>');