CREATE TABLE entity_links
(
    entity_link_def_id    CHAR(10)  NOT NULL  PRIMARY KEY,
    entity_source_id      CHAR(10)  NOT NULL,
    entity_target_id      CHAR(10)  NOT NULL,
    CONSTRAINT entity_link_def_fk FOREIGN KEY(entity_link_def_id) REFERENCES entity_link_defs(id),
    CONSTRAINT entity_source_fk   FOREIGN KEY(entity_source_id)   REFERENCES entities(id),
    CONSTRAINT entity_target_fk   FOREIGN KEY(entity_target_id)   REFERENCES entities(id)
);
