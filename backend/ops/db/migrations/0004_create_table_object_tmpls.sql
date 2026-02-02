CREATE TABLE object_tmpls
(
    id                      CHAR(10)      PRIMARY KEY,
    name                    VARCHAR(64)   NOT NULL,
    description             VARCHAR(256),
    listing_attr_tmpl_id    CHAR(10)      NOT NULL,

    CONSTRAINT obj_tmpl_listing_attr_tmpl_fk  FOREIGN KEY(listing_attr_tmpl_id) REFERENCES attribute_tmpls(id)
);
