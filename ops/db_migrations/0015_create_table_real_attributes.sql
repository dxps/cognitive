CREATE TABLE real_attributes
(
    id             CHAR(10)              PRIMARY KEY,
    def_id         CHAR(10),
    owner_type     CHAR(3),
    value          REAL,
    CONSTRAINT def_fk FOREIGN KEY(def_id) REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN text_attributes.def_id is 'The definition id of this attribute.';
