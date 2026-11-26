/** One watched form value next to the persisted value it started from. */
export interface UnsavedField {
    value: unknown;
    original: unknown;
}

/**
 * True when any watched field differs from its persisted value. Arrays and objects are compared
 * by content, a form that edits a list must not look clean because the array identity changed.
 */
export function hasUnsavedChanges(fields: UnsavedField[]): boolean {
    return fields.some(({ value, original }) => !isSameValue(value, original));
}

function isSameValue(value: unknown, original: unknown): boolean {
    if (value === original) {
        return true;
    }
    if (typeof value === 'object' && typeof original === 'object') {
        return JSON.stringify(value) === JSON.stringify(original);
    }
    return false;
}
