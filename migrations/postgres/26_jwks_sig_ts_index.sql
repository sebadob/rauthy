CREATE INDEX jwks_signature_created_at_index
    ON jwks (signature, created_at);