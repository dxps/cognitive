CREATE TABLE attribute_defs
(
    id              CHAR(10)      PRIMARY KEY,
    name            VARCHAR(64)   NOT NULL,
    description     VARCHAR(256),
    value_type      VARCHAR(16)   NOT NULL,
    default_value   VARCHAR(20),
    multivalued     BOOLEAN       DEFAULT false,
    composite       BOOLEAN       DEFAULT false,
    value_rules     VARCHAR(256),
    required        BOOLEAN       DEFAULT false,
    tag_id          CHAR(10),
    CONSTRAINT tag_fk FOREIGN KEY(tag_id) REFERENCES tags(id)
);

COMMENT ON COLUMN attribute_defs.composite is 
    'It tells if the attribute is a composite one. If true, it means that it includes other attributes';
