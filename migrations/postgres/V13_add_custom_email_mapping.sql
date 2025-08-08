ALTER TABLE clients
    ADD cust_email_mapping TEXT
        REFERENCES user_attr_config
        ON UPDATE ON DELETE SET NULL;
