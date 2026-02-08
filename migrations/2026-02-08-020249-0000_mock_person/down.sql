DELETE FROM persons
WHERE name ~ '^Pessoa [0-9]{4}$';

DROP FUNCTION IF EXISTS make_cpf(integer);
