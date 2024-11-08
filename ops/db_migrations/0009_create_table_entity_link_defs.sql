CREATE TYPE link_cardinality AS ENUM ('one-to-one', 'one-to-many', 'many-to-many');

CREATE TABLE entity_link_defs
(
    id                      CHAR(10)         PRIMARY KEY,
    name                    VARCHAR(32),
    description             VARCHAR(256),
    cardinality             link_cardinality,
    source_entity_def_id    CHAR(10)         NOT NULL,
    target_entity_def_id    CHAR(10)         NOT NULL,
    CONSTRAINT source_entity_def_fk    FOREIGN KEY(source_entity_def_id)    REFERENCES entity_defs(id),
    CONSTRAINT target_entity_def_fk    FOREIGN KEY(target_entity_def_id)    REFERENCES entity_defs(id)
);
