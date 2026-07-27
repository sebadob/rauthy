UPDATE users
SET email = 'admin@localhost'
WHERE email = 'admin@localhost.de';

-- These are just for internal integration tests.
-- They will be removed in any case when doing a production setup
-- and are changed only for consistency.
UPDATE users
SET email = 'init_admin@localhost'
WHERE email = 'init_admin@localhost.de';

UPDATE users
SET email = 'test_admin@localhost'
WHERE email = 'test_admin@localhost.de';
