import { fetchGet } from '$api/fetch';
import { isBrowser } from '$utils/helpers';
import type { I18nConfigResponse, Language } from '$api/types/i18n.ts';

let _config: undefined | I18nConfigResponse = $state();

export function useI18nConfig() {
    if (!_config && isBrowser()) {
        fetchGet<I18nConfigResponse>('/auth/v1/i18n_config').then(res => {
            if (res.body) {
                let cfg = res.body;
                cfg.common = res.body.common.map(map_language);
                cfg.admin = res.body.admin.map(map_language);
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
    };
}

const map_language = (l: Language): Language => {
    // @ts-ignore
    return l === 'zhhans' ? 'zh' : l;
}
