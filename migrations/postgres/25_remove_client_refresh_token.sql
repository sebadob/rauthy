-- modify the clients table and remove obsolete `refresh_token` column
-- the new logic will just take a look at the flows_enabled value

alter table clients
    drop column refresh_token;
