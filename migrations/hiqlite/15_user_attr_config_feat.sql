ALTER TABLE user_attr_config
    ADD default_value BLOB;

ALTER TABLE user_attr_config
    ADD typ TEXT;

ALTER TABLE user_attr_config
    ADD user_editable INTEGER DEFAULT false NOT NULL;

CREATE INDEX user_attr_config_user_editable_index
    ON user_attr_config (user_editable);
