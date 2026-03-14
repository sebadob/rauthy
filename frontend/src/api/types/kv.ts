import type { JsonValue } from '$utils/jsonValue';

export interface KVNamespaceRequest {
    /// Validation: PATTERN_GROUP
    name: string;
}

export interface KVAccessRequest {
    enabled: boolean;
    name?: string;
}

export interface KVValueRequest {
    /// Validation: PATTERN_GROUP
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

/// The `Authorization` header must be `Bearer {id}${secret}`
export interface KVAccessResponse {
    id: string;
    ns: string;
    secret: string;
    enabled: boolean;
    name?: string;
}

export interface KVValueResponse {
    key: string;
    encrypted: boolean;
    value: JsonValue;
}
