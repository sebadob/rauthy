let _lang = $state('');

export function initLang() {
    if (typeof document !== 'undefined') {
        let l = document.documentElement.lang.toUpperCase().slice(0, 2).toLowerCase();
        console.log('lang in gs', l);
        _lang = l;
    } else {
        _lang = 'en';
    }
}

export function useLang(): string {
    return _lang;
}
