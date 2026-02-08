CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION make_cpf(i integer)
RETURNS text
LANGUAGE plpgsql
IMMUTABLE
STRICT
AS $$
DECLARE
    base text;
    sum int;
    r int;
    d1 int;
    d2 int;
    digit int;
    j int;
    cpf11 text;
BEGIN
    base := lpad(((i * 7919) % 1000000000)::text, 9, '0');

    sum := 0;
    FOR j IN 1..9 LOOP
        digit := substr(base, j, 1)::int;
        sum := sum + digit * (11 - j);
    END LOOP;
    r := sum % 11;
    d1 := CASE WHEN r < 2 THEN 0 ELSE 11 - r END;

    sum := 0;
    FOR j IN 1..9 LOOP
        digit := substr(base, j, 1)::int;
        sum := sum + digit * (12 - j);
    END LOOP;
    sum := sum + d1 * 2;
    r := sum % 11;
    d2 := CASE WHEN r < 2 THEN 0 ELSE 11 - r END;

    cpf11 := base || d1::text || d2::text;

    -- Formata: 000.000.000-00
    RETURN substr(cpf11, 1, 3) || '.' ||
           substr(cpf11, 4, 3) || '.' ||
           substr(cpf11, 7, 3) || '-' ||
           substr(cpf11,10, 2);
END;
$$;

INSERT INTO persons (id, name, cpf)
SELECT
    gen_random_uuid(),
    'Pessoa ' || lpad(gs::text, 4, '0'),
    make_cpf(gs)
FROM generate_series(1, 1000) AS gs;
