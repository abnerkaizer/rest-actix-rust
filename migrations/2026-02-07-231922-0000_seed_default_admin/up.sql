-- (opcional, mas recomendado) role só pode ser admin/user
ALTER TABLE users
  ALTER COLUMN role SET DEFAULT 'user';

INSERT INTO users (id, email, password_hash, created_at, role)
SELECT
  gen_random_uuid(),
  'admin@local',
  '$2b$12$ydFet0MREnT9HachT5dlAe7HXL8.X0CCndRqbl.HeIsGsHBKpRBCS',
  NOW(),
  'admin'
WHERE NOT EXISTS (
  SELECT 1 FROM users WHERE email = 'admin@local'
);
