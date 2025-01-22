import {goto} from "$app/navigation";

// Record<> or Map<> do not work reactively in this case
let _signals: any = $state({});
// needs to be undefined in the beginning (and therefore produce a bit of
// ugly syntax in useParam()), to make pre-rendering work properly
// -> `window` is not available on the server
let _params: undefined | URLSearchParams = typeof window !== 'undefined'
    ? new URLSearchParams(window.location.search)
    : undefined;

export const updateParams = (paramsNew: URLSearchParams) => {
    for (let [key, value] of paramsNew.entries()) {
        if (_params && _params.get(key) !== value) {
            // console.log('updating ', key, ' from ', _params.get(key), ' to ', value);
            _signals[key] = value;
        }
    }
    _params = paramsNew;
}

export interface IParam {
    get: () => undefined | string,
    getNum: () => undefined | number,
    true: () => boolean,
    set: (value?: string | number | boolean) => void,
}

export const useParam = (key: string, initialValue?: string | number | boolean): IParam => {
    if (!_params && typeof window !== 'undefined') {
        _params = new URLSearchParams(window.location.search);
        for (let [key, value] of _signals.entries()) {
            _params.set(key, value);
        }
    }

    let param = _signals[key];
    if (!param) {
        let current = _params?.get(key);
        if (current) {
            _signals[key] = current;
            if (_params) {
                _params.set(key, current);
            }
        } else if (initialValue) {
            _signals[key] = initialValue;
            if (_params) {
                _params.set(key, initialValue.toString());
                go();
            }
        }
    }

    return {
        get(): undefined | string {
            return _signals[key];
        },
        getNum(): undefined | number {
            let num = Number.parseInt(_signals[key]);
            if (!isNaN(num)) {
                return num;
            }
        },
        true(): boolean {
            return _signals[key] === true || _signals[key] === 'true';
        },
        set(value?: string | number | boolean) {
            if (value) {
                _signals[key] = value.toString();
                if (_params) {
                    _params.set(key, value.toString());
                }
            } else {
                _signals[key] = undefined;
                if (_params) {
                    _params.delete(key);
                }
            }
            go();
        },
    }
}

function go() {
    if (typeof window !== 'undefined' && _params) {
        let params = _params.toString().replace(' ', '+');
        goto(`?${params}`, {})
            .catch(err => console.error(err));
    }
}
