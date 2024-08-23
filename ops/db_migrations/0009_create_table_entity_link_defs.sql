CREATE TYPE link_cardinality AS ENUM ('one-to-one', 'one-to-many', 'many-to-many');

CREATE TABLE entity_link_defs
(
    id                      CHAR(10)         PRIMARY KEY,
    name                    VARCHAR(32),
    description             VARCHAR(256),
    cardinality             link_cardinality,
    entity_def_source_id    CHAR(10)         NOT NULL,
    entity_def_target_id    CHAR(10)         NOT NULL,
    CONSTRAINT entity_def_source_fk    FOREIGN KEY(entity_def_source_id)    REFERENCES entity_defs(id),
    CONSTRAINT entity_def_target_fk    FOREIGN KEY(entity_def_target_id)    REFERENCES entity_defs(id)
);
