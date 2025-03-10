import {fetchGet} from "$api/fetch";
import {isBrowser} from "$utils/helpers.ts";
import type {I18nConfigResponse, Language} from "$api/types/i18n.ts";

let _config: undefined | I18nConfigResponse = $state();

export function useI18nConfig() {
    if (!_config && isBrowser()) {
        fetchGet<I18nConfigResponse>('/auth/v1/i18n_config')
            .then(res => {
                if (res.body) {
                    res.body.common.map(l => {
                        if (l === 'zhhans') {
                            return 'zh';
                        } else {
                            return l;
                        }
                    });
                    // res.body.admin.map(l => {
                    //     if (l === 'zhhans') {
                    //         return 'zh';
                    //     } else {
                    //         return l;
                    //     }
                    // });
                    _config = res.body;
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
