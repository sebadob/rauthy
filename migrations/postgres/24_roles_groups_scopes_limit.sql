-- remove the limit checks on id and name fields for roles, groups, scopes
-- to allow longer scope names like i.e. 'urn:ietf:params:oauth:grant-type:device_code'

-- noop on postgres -> has been done with past migrations already