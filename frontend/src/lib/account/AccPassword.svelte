<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { fade } from 'svelte/transition';
    import AccModPwd from '$lib5/account/AccModPwd.svelte';
    import WebauthnRequest from '$lib5/WebauthnRequest.svelte';
    import CheckIcon from '$lib5/CheckIcon.svelte';
    import { useI18n } from '$state/i18n.svelte.js';
    import type { UpdateUserSelfRequest, UserResponse } from '$api/types/user.ts';
    import type { AuthProviderTemplate } from '$api/templates/AuthProvider.ts';
    import IconCheck from '$icons/IconCheck.svelte';
    import type { PropsPassword } from './props.ts';
    import { fetchGet, fetchPost, fetchPut } from '$api/fetch';
    import type { MfaPurpose, WebauthnAdditionalData } from '$webauthn/types.ts';
    import type { PasskeyResponse } from '$api/types/webauthn.ts';
    import { onMount } from 'svelte';
    import type { RequestResetRequest } from '$api/types/authorize.ts';
    import { fetchSolvePow } from '$utils/pow';

    let {
        user = $bindable(),
        authProvider,
        viewModePhone,
    }: {
        user: UserResponse;
        authProvider: undefined | AuthProviderTemplate;
        viewModePhone?: boolean;
    } = $props();

    let t = useI18n();

    let inputWidth = $derived(viewModePhone ? 'calc(100vw - 1.5rem)' : '300px');

    let accType = $state(user.account_type);
    let passkeys: PasskeyResponse[] = $state([]);
    let isPwdValid: undefined | (() => boolean) = $state();
    let convertAccount = $state(false);
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let mfaPurpose: undefined | MfaPurpose = $state();

    let passwords: PropsPassword = $state({
        current: '',
        new: '',
        newConfirm: '',
    });

    let canConvertToPasskey = $derived(passkeys.filter(pk => pk.user_verified).length > 0);

    onMount(() => {
        fetchPasskeys();
    });

    async function fetchPasskeys() {
        let res = await fetchGet<PasskeyResponse[]>(`/auth/v1/users/${user.id}/webauthn`);
        if (res.body) {
            passkeys = res.body;
        } else {
            console.error('error fetching passkeys: ' + res.error);
        }
    }

    async function convertToPasskeyOnly() {
        let res = await fetchPost<UserResponse>(`/auth/v1/users/${user.id}/self/convert_passkey`);
        if (!res.error) {
            window.location.reload();
        } else {
            console.error('error fetching passkeys: ' + res.error);
        }
    }

    async function onSubmit() {
        err = '';
        if (passkeys.length > 0) {
            await onSubmitMfa();
        } else {
            await onSubmitFinish();
        }
    }

    async function onSubmitMfa() {
        if (!isPwdValid?.()) {
            err = t.common.invalidInput;
            return;
        }
        mfaPurpose = 'PasswordNew';
    }

    async function onSubmitFinish(mfaCode?: string) {
        if (!isPwdValid?.()) {
            err = t.common.invalidInput;
            return;
        }

        isLoading = true;

        let payload: UpdateUserSelfRequest = {
            password_new: passwords.new,
        };
        if (mfaCode) {
            payload.mfa_code = mfaCode;
        } else {
            payload.password_current = passwords.current;
        }

        let res = await fetchPut<UserResponse>(`/auth/v1/users/${user.id}/self`, payload);
        if (res.body) {
            success = true;
            passwords = {
                current: '',
                new: '',
                newConfirm: '',
            };
            user = res.body;
            accType = res.body.account_type;

            setTimeout(() => {
                success = false;
                convertAccount = false;
            }, 3000);
        } else {
            err = res.error?.message || 'Error';
        }

        isLoading = false;
    }

    function onWebauthnError(error: string) {
        mfaPurpose = undefined;
        err = error;
        setTimeout(() => {
            err = '';
        }, 5000);
    }

    function onWebauthnSuccess(data?: WebauthnAdditionalData) {
        mfaPurpose = undefined;
        if (data && 'code' in data) {
            onSubmitFinish(data.code);
        }
    }

    async function requestPasswordReset() {
        let pow = (await fetchSolvePow()) || '';
        let payload: RequestResetRequest = {
            pow,
            email: user.email,
        };
        let res = await fetchPost('/auth/v1/users/request_reset', payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
        }
    }
</script>

<div class="container">
    {#if mfaPurpose}
        <WebauthnRequest
            userId={user.id}
            purpose={mfaPurpose}
            onSuccess={onWebauthnSuccess}
            onError={onWebauthnError}
        />
    {/if}

    {#if accType === 'federated'}
        <div class="m-05">
            <p>{t.account.federatedConvertPassword1}</p>
            <p><b>{authProvider?.name || 'UNKNOWN'}</b></p>
            <div style:height=".3rem"></div>
            <p>{t.account.federatedConvertPassword2}</p>
            {#if success}
                <CheckIcon checked />
            {:else}
                <Button level={2} onclick={requestPasswordReset}>
                    {t.account.passwordReset}
                </Button>
            {/if}
        </div>
    {/if}

    {#if (accType === 'passkey' || accType === 'federated_passkey') && !convertAccount}
        <p>{t.account.accTypePasskeyText1}</p>
        <p>{t.account.accTypePasskeyText2}</p>
        <p>{t.account.accTypePasskeyText3}</p>
        <div>
            <Button level={2} onclick={() => (convertAccount = true)}>
                {t.account.convertAccount}
            </Button>
        </div>
    {/if}

    {#if accType === 'password' || accType === 'federated_password' || convertAccount}
        <div>
            <AccModPwd
                bind:passwords
                bind:isValid={isPwdValid}
                {inputWidth}
                hideCurrentPassword={!(accType === 'password' && passkeys.length < 1)}
            />

            <div class="save">
                <Button onclick={onSubmit} level={1} {isLoading}>
                    {t.common.save}
                </Button>
                {#if success}
                    <div class="success" transition:fade>
                        <IconCheck />
                    </div>
                {:else if err}
                    <div class="err" transition:fade>
                        {err}
                    </div>
                {:else if convertAccount && !isLoading}
                    <div class="cancel">
                        <Button level={3} onclick={() => (convertAccount = false)}>
                            {t.common.cancel}
                        </Button>
                    </div>
                {/if}
            </div>
        </div>

        {#if !convertAccount && canConvertToPasskey}
            <div class="convertPasskey">
                <h3>{t.account.convertAccount}</h3>
                <p>{t.account.convertAccountP1}</p>
                <Button level={2} onclick={convertToPasskeyOnly}>
                    {t.account.convertAccount}
                </Button>
            </div>
        {/if}
    {/if}
</div>

<style>
    .container {
        padding: 0 5px;
        display: flex;
        flex-direction: column;
        overflow-x: clip;
    }

    .convertPasskey {
        margin: 1rem 0;
    }

    .m-05 {
        margin: 0.5rem;
    }

    .save {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.66rem;
    }

    .success {
        margin-bottom: -0.25rem;
        color: var(--action);
    }
</style>
