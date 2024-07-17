CREATE TABLE entity_attrs
(
    id                CHAR(10)      PRIMARY KEY,
    def_id            CHAR(10),
    entity_id         CHAR(10),
    name              VARCHAR(64)   NOT NULL,
    value             VARCHAR(1024),
    CONSTRAINT def_fk FOREIGN KEY(def_id) REFERENCES entity_defs(id)
);

COMMENT ON COLUMN entity_attrs.def_id is 'The definition id of this attribute.';
COMMENT ON COLUMN entity_attrs.entity_id is 'The entity id that has this attribute.';
