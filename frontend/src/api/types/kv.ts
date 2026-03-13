export interface KVNamespaceRequest {
    /// Validation: PATTERN_ALNUM_48
    name: string;
}

export interface KVAccessRequest {
    enabled: boolean;
    name?: string;
}

export interface KVValueRequest {
    /// Validation: PATTERN_ALNUM_48
    key: string;
    /// If set to `true`, the backend will encrypt the value on the application layer.
    /// The database will only contain encrypted data. Requires more resources, but is
    /// probably a good idea for very sensitive information.
    encrypted: boolean;
    value: JsonValue;
}

export interface KVNamespaceResponse {
    name: string;
}

export interface KVAccessResponse {
    id: string;
    ns: string;
    enabled: boolean;
    name?: string;
}

export interface KVValueResponse {
    key: string;
    encrypted: boolean;
    value: JsonValue;
}
