CREATE TABLE text_attributes
(
    owner_id             CHAR(10),
    owner_type           CHAR(3),
    def_id               CHAR(10),
    value                TEXT,
    PRIMARY KEY (owner_id, owner_type, def_id),
    CONSTRAINT text_attributes___def_fk        FOREIGN KEY (def_id)   REFERENCES attribute_defs(id),
    CONSTRAINT text_attributes___entities___fk FOREIGN KEY (owner_id) REFERENCES public.entities(id) ON DELETE CASCADE
);

COMMENT ON COLUMN text_attributes.def_id     is 'The definition id of this attribute.';
COMMENT ON COLUMN text_attributes.owner_type is 'The type of the owner (modeled in code as ItemType enum).';
