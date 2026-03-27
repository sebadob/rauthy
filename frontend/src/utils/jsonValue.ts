export type JsonValue = JSON | Array<JsonValue> | string | number | boolean | undefined | null;

// Parses a `string` into any valid JSON value. This will match
// `serde_json::Value` in the backend. If all typed parsing fails,
// `string` will be the fallback.
export function parseJsonValue(value: string | undefined): JsonValue {
    value = value?.trim();
    if (!value) {
        return undefined;
    }

    if (value === 'true' || value === 'false') {
        return value === 'true';
    }

    let num = Number.parseInt(value);
    if (!isNaN(num)) {
        return num;
    }

    if (value.startsWith('[')) {
        // to make our life a lot easier, we will try to convert
        // it into valid JSON for automatic parsing
        try {
            let json = JSON.parse(`{"arr": ${value}}`);
            return json.arr;
        } catch (e) {}
    }

    try {
        return JSON.parse(value);
    } catch (e) {}

    return value;
}

/// Converts any `JsonValue` from the backend into a `string`.
/// Keeps quotes and parenthesis for Arrays and Objects.
export function stringifyJsonValue(value: JsonValue, space: number = 4): string {
    if (!value) {
        return '';
    }
    if (typeof value === 'object') {
        return JSON.stringify(value, undefined, space);
    }
    if (Array.isArray(value)) {
        return '[' + (value as Array<JsonValue>).map(v => JSON.stringify(v)).join(', ') + ']';
    }
    return String(value);
}
