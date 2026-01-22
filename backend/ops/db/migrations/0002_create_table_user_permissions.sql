CREATE TABLE user_permissions
(
    user_id         CHAR(10)      NOT NULL,
    permission      VARCHAR(256)  NOT NULL,

    CONSTRAINT user_unique_user_id_permission UNIQUE (user_id, permission),
    CONSTRAINT user_fk FOREIGN KEY(user_id) REFERENCES user_accounts(id)
);

