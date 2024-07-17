CREATE TABLE entity_defs_attribute_defs_xref
(
    entity_def_id      CHAR(10),
    attribute_def_id   CHAR(10),
    CONSTRAINT entity_def_fk    FOREIGN KEY(entity_def_id)    REFERENCES entity_defs(id),
    CONSTRAINT attribute_def_fk FOREIGN KEY(attribute_def_id) REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN entity_defs_attribute_defs_xref.attribute_def_id is 'The definition id of this attribute.';
COMMENT ON COLUMN entity_defs_attribute_defs_xref.entity_def_id is 'The definition id of the entity that has this attribute.';
