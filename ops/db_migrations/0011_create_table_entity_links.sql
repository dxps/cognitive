CREATE TABLE entity_links
(
    def_id         CHAR(10)            NOT NULL,
    source_id      CHAR(10)            NOT NULL,
    target_id      CHAR(10)            NOT NULL,
    PRIMARY KEY (def_id, source_id, target_id),
    CONSTRAINT entity_link_def_fk      FOREIGN KEY(def_id)      REFERENCES entity_link_defs(id),
    CONSTRAINT entity_link_source_fk   FOREIGN KEY(source_id)   REFERENCES entities(id),
    CONSTRAINT entity_link_target_fk   FOREIGN KEY(target_id)   REFERENCES entities(id)
);
