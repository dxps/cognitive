CREATE TABLE attribute_defs
(
    id              CHAR(10)      PRIMARY KEY,
    name            VARCHAR(64)   NOT NULL,
    description     VARCHAR(128),
    data_type       VARCHAR(16)   NOT NULL,
    default_value   VARCHAR(20),
    is_required     BOOLEAN       DEFAULT false,
    tag_id          CHAR(10),
    CONSTRAINT tag_fk FOREIGN KEY(tag_id) REFERENCES tags(id)
);
