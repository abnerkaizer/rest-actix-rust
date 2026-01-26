DELETE FROM persons a
USING persons b
WHERE a.cpf = b.cpf
  AND a.id > b.id;

ALTER TABLE persons
ADD CONSTRAINT persons_cpf_unique UNIQUE (cpf);