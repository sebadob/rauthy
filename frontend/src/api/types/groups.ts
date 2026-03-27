import type { JsonValue } from '$utils/jsonValue';

export interface GroupRequest {
    /// Validation: PATTERN_GROUP
    group: string;
    json_meta?: JsonValue;
}

export interface GroupResponse {
    id: string;
    name: string;
    json_meta?: JsonValue;
}
