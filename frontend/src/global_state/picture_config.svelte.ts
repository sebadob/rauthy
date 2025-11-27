import { fetchGet } from '$api/fetch';
import { isBrowser } from '$utils/helpers';
import type { UserPictureConfig } from '$api/types/user.ts';

let _config: undefined | UserPictureConfig = $state();

export function usePictureConfig() {
    if (!_config && isBrowser()) {
        fetchGet<UserPictureConfig>('/auth/v1/users/picture_config').then(res => {
            _config = res.body;
        });
    }

    return {
        get(): undefined | UserPictureConfig {
            return _config;
        },
    };
}
