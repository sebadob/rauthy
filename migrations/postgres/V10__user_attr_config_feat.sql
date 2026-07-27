ALTER TABLE user_attr_config
    ADD default_value BYTEA;

ALTER TABLE user_attr_config
    ADD typ VARCHAR;

ALTER TABLE user_attr_config
    ADD user_editable BOOL DEFAULT false NOT NULL;

CREATE INDEX user_attr_config_user_editable_index
    ON user_attr_config (user_editable);
