import {fetchGet} from "$api/fetch";
import {isBrowser} from "$utils/helpers";
import type {I18nConfigResponse, Language} from "$api/types/i18n.ts";

let _config: undefined | I18nConfigResponse = $state();

export function useI18nConfig() {
    if (!_config && isBrowser()) {
        fetchGet<I18nConfigResponse>('/auth/v1/i18n_config')
            .then(res => {
                if (res.body) {
                    let cfg = res.body;
                    cfg.common = res.body.common.map(l => {
                        // @ts-ignore
                        if (l === 'zhhans') {
                            return 'zh';
                        } else {
                            return l;
                        }
                    });
                    _config = cfg;
                }
            });
    }

    return {
        admin(): undefined | Language[] {
            return _config?.admin;
        },
        common(): undefined | Language[] {
            return _config?.common;
        },
    }
}
