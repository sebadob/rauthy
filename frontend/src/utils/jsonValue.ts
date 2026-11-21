export type JsonValue = JSON | Array<JsonValue> | string | number | boolean | undefined | null;

// These fields accept JSON values and unquoted text.
export function parseJsonValue(value: string | undefined): JsonValue {
    value = value?.trim();
    if (!value) {
        return undefined;
    }

    try {
        return JSON.parse(value);
    } catch {
        return value;
    }
}

export function stringifyJsonValue(value: JsonValue, space: number = 4): string {
    if (value === undefined) {
        return '';
    }
    // Keep plain text readable and quote strings that would parse as another value.
    if (typeof value === 'string' && parseJsonValue(value) === value) {
        return value;
    }
    return JSON.stringify(value, undefined, space);
}
