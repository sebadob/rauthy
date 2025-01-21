let _isDev = $state(typeof (process) !== 'undefined' && process.env.DEV_MODE === 'true');

export function initIsDev() {
    _isDev = process.env.DEV_MODE === 'true'
    // if (typeof (process) !== 'undefined') {
    //     _isDev = process.env.DEV_MODE === 'true';
    // } else if (typeof (window) !== 'undefined') {
    //     // We can take advantage of the fast that SSR template values will not exist during local dev.
    //     // TODO needs adjustment as soon as this has been converted into a template
    //     let elem = window.document.getElementById(TPL_CS);
    //     _isDev = elem.id === '{{ csrf_token }}';
    // }
}

export function useIsDev() {
    return {
        get(): boolean {
            return _isDev;
        },
        set(value: boolean) {
            _isDev = value;
        }
    }
}