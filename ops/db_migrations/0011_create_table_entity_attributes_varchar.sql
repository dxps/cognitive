CREATE TABLE entity_attributes_varchar
(
    id                CHAR(10)      PRIMARY KEY,
    entity_id         CHAR(10),
    attribute_def_id  CHAR(10),
    attribute_value   VARCHAR(1024),
    CONSTRAINT entity_fk    FOREIGN KEY(entity_id)        REFERENCES entities(id),
    CONSTRAINT attribute_fk FOREIGN KEY(attribute_def_id) REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN entity_attributes_varchar.entity_id        is 'The entity id that has this attribute.';
COMMENT ON COLUMN entity_attributes_varchar.attribute_def_id is 'The definition id of this attribute.';
