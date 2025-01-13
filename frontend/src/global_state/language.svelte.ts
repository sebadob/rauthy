let _lang = $state('');

export function initLang() {
    if (typeof document !== 'undefined') {
        _lang = document.documentElement.lang.slice(0, 2).toLowerCase();
    } else {
        _lang = 'en';
    }
}

export function useLang(): string {
    return _lang;
}
