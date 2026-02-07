DELETE FROM users WHERE email = 'admin@local';

ALTER TABLE users DROP CONSTRAINT IF EXISTS users_role_check;
ALTER TABLE users ALTER COLUMN role DROP DEFAULT;
