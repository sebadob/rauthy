<script lang="ts">
    import { onMount } from 'svelte';
    import { redirectToLogin } from '#utils/helpers.ts';
    import Input from '#lib/form/Input.svelte';
    import Button from '#lib/button/Button.svelte';
    import { TPL_DEVICE_USER_CODE_LENGTH } from '#utils/constants.ts';
    import { fetchSolvePow } from '#utils/pow.ts';
    import PublicPage from '#lib/PublicPage.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Template from '#lib/Template.svelte';
    import { useParam } from '#state/param.svelte.ts';
    import type { SessionResponse } from '#api/types/session.ts';
    import { fetchGet, fetchPost } from '#api/fetch.ts';
    import { PATTERN_ALNUM } from '#utils/patterns.ts';
    import type { DeviceVerifyRequest, DeviceVerifyResponse } from '#api/types/device.ts';

    let t = useI18n();
    let session: undefined | SessionResponse = $state();

    let err = $state('');
    let isError = $state(false);
    let isLoading = $state(false);
    let userCodeLength = $state(8);

    let scopes: undefined | string[] = $state();
    let isAccepted = $state(false);
    let isDeclined = $state(false);

    let userCode = $state('');

    onMount(async () => {
        let param = useParam('code');
        let code = param.get() || '';
        userCode = code;

        // 'noRedirect': the device flow is opened by users that are not logged in, so a 401 here is
        // the normal case. The default 'handle401' would reload the document on every load, which
        // never ends - the redirect below is what this page does with a missing session anyway.
        let res = await fetchGet<SessionResponse>(
            '/auth/v1/oidc/sessioninfo',
            'json',
            'noRedirect',
        );
        if (res.body) {
            if (res.body.state != 'Auth') {
                redirectToLogin(`device?code=${code}`);
            }
            session = res.body;
        } else if (code) {
            redirectToLogin(`device?code=${code}`);
        } else {
            redirectToLogin('device');
        }
    });

    async function onSubmit(deviceAccepted: 'accept' | 'decline' | 'pending') {
        err = '';

        if (isError) {
            return;
        }
        if (userCode.length < userCodeLength) {
            err = t.common.errTooShort;
            return;
        }
        if (userCode.length > userCodeLength) {
            err = t.common.errTooLong;
            return;
        }
        isLoading = true;

        let pow = await fetchSolvePow();
        if (!pow) {
            err = 'PoW error - please contact your administrator';
            return;
        }
        let payload: DeviceVerifyRequest = {
            user_code: userCode,
            pow,
            device_accepted: deviceAccepted,
        };
        let res = await fetchPost<DeviceVerifyResponse>('/auth/v1/oidc/device/verify', payload);
        if (res.status === 200) {
            scopes = res.body?.scopes?.split(' ') || ['openid'];
        } else if (res.status === 202) {
            isAccepted = true;
            setTimeout(() => {
                window.location.replace('/auth/v1/account?v=devices');
            }, 2000);
        } else if (res.status === 204) {
            isDeclined = true;
        } else if (res.status === 404) {
            err = t.device.wrongOrExpired;
        } else {
            console.error(res);
            err = res.error?.message || '';
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t?.device.title || 'Device Authorization'}</title>
</svelte:head>

<Template id={TPL_DEVICE_USER_CODE_LENGTH} bind:value={userCodeLength} />

<PublicPage>
    {#if session}
        <div class="public-card">
            <h2>{t.device.title}</h2>

            {#if scopes === undefined}
                <div class="desc">
                    {t.device.desc.replace('{{count}}', userCodeLength.toString())}
                </div>

                <Input
                    name="userCode"
                    autocomplete="off"
                    label={t.device.userCode}
                    placeholder={t.device.userCode}
                    bind:value={userCode}
                    bind:isError
                    required
                    min={userCodeLength.toString()}
                    max={userCodeLength.toString()}
                    pattern={PATTERN_ALNUM}
                />

                <div class="btn">
                    <Button onclick={() => onSubmit('pending')} {isLoading}>
                        {t.device.submit}
                    </Button>
                </div>
            {:else if isAccepted}
                <div class="desc">
                    <p>{t.device.isAccepted}</p>
                    <p>{t.device.autoRedirectAccount}</p>
                </div>
            {:else if isDeclined}
                <div class="desc">
                    <p class="declined">{t.device.isDeclined}</p>
                    <p>{t.device.closeWindow}</p>
                </div>
            {:else}
                <div class="desc">
                    {t.device.descScopes}
                    <ul>
                        {#each scopes as scope}
                            <li>{scope}</li>
                        {/each}
                    </ul>
                </div>

                <div class="inline">
                    <Button onclick={() => onSubmit('accept')} {isLoading}>
                        {t.device.accept}
                    </Button>
                    <Button level={-1} onclick={() => onSubmit('decline')} {isLoading}>
                        {t.device.decline}
                    </Button>
                </div>
            {/if}

            {#if err}<div class="err">{err}</div>{/if}
        </div>
    {/if}
</PublicPage>

<style>
    ul {
        margin-top: 0.5rem;
    }

    li {
        margin-left: 1rem;
    }

    .btn {
        margin-top: 0.5rem;
    }

    .declined {
        color: hsl(var(--error));
    }

    .desc {
        margin-block: 0.75rem;
    }

    .err {
        color: hsl(var(--error));
    }

    .inline {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
    }
</style>
