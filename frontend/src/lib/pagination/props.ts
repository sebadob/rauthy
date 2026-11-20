import { isBrowser } from '#utils/helpers.ts';

export type PageSize = 5 | 10 | 15 | 20 | 30 | 50 | 100 | 500 | 1000;

export const PAGE_SIZE_DEFAULT: PageSize = 20;

export const PAGE_SIZE_OPTIONS: PageSize[] = [5, 10, 20, 30, 50, 100, 500, 1000];

// the same key for every pagination, so the chosen count is shared across the whole Admin UI
const PAGE_SIZE_KEY = 'pageSize';

/// The last chosen entries-per-page, `PAGE_SIZE_DEFAULT` when nothing is saved.
export function loadPageSize(): PageSize {
    if (!isBrowser()) {
        return PAGE_SIZE_DEFAULT;
    }
    let saved = Number.parseInt(localStorage.getItem(PAGE_SIZE_KEY) ?? '');
    return PAGE_SIZE_OPTIONS.find(opt => opt === saved) ?? PAGE_SIZE_DEFAULT;
}

/// Stores the entries-per-page, restored by `loadPageSize` on reload and page switch.
export function savePageSize(size: PageSize) {
    if (isBrowser()) {
        localStorage.setItem(PAGE_SIZE_KEY, size.toString());
    }
}
