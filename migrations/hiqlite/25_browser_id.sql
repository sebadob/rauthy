ALTER TABLE login_locations
    ADD browser_id TEXT;

CREATE INDEX login_locations_browser_id_index
    ON login_locations (browser_id);
