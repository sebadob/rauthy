import { fetchGet } from '$api/fetch';
import { isBrowser } from '$utils/helpers';
import type { UserPictureConfig } from '$api/types/user.ts';

let _config: undefined | UserPictureConfig = $state();
let isFetching = false;

export function usePictureConfig() {
    if (!isFetching && !_config && isBrowser()) {
        isFetching = true;
        fetchGet<UserPictureConfig>('/auth/v1/users/picture_config')
            .then(res => {
                _config = res.body;
                isFetching = false;
            })
            .catch(err => {
                console.error('cannot fetch picture_config', err);
                isFetching = false;
            });
    }

    return {
        get(): undefined | UserPictureConfig {
            return _config;
        },
    };
}
