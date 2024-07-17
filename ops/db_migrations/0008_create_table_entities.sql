CREATE TABLE entities
(
    id               CHAR(10)      PRIMARY KEY,
    def_id           CHAR(10),
    CONSTRAINT def_fk FOREIGN KEY(def_id) REFERENCES entity_defs(id)
);

COMMENT ON COLUMN entities.def_id is 'The definition id of this entity.';
