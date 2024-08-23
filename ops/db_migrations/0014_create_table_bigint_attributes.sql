CREATE TABLE bigint_attributes
(
    owner_id             CHAR(10),
    owner_type           CHAR(3),
    def_id               CHAR(10),
    value                BIGINT,
    PRIMARY KEY (owner_id, owner_type, def_id),
    CONSTRAINT def_fk    FOREIGN KEY(def_id)   REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN text_attributes.def_id is 'The definition id of this attribute.';
