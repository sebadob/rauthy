import {DEV_MODE} from "$env/static/public";

let _isDev = $state(DEV_MODE === 'true');

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