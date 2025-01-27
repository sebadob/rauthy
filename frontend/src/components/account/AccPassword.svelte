<script lang="ts">
    import Button from "$lib5/Button.svelte";
    import {fade} from 'svelte/transition';
    import AccModPwd from "./AccModPwd.svelte";
    import {postPasswordResetRequest} from "$utils/dataFetching.js";
    import WebauthnRequest from "../webauthn/WebauthnRequest.svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {UpdateUserSelfRequest, UserResponse} from "$api/types/user.ts";
    import type {AuthProviderTemplate} from "$api/templates/AuthProvider.ts";
    import IconCheck from "$icons/IconCheck.svelte";
    import type {PropsPassword} from "./props.ts";
    import {fetchGet, fetchPost, fetchPut} from "$api/fetch.ts";
    import type {MfaPurpose, WebauthnAdditionalData} from "$webauthn/types.ts";
    import type {PasskeyResponse} from "$api/types/webauthn.ts";
    import {onMount} from "svelte";

    let {
        user = $bindable(),
        authProvider,
        viewModePhone,
    }: {
        user: UserResponse,
        authProvider: undefined | AuthProviderTemplate,
        viewModePhone?: boolean,
    } = $props();

    let t = useI18n();

    let inputWidth = $derived(viewModePhone ? 'calc(100vw - 1.5rem)' : '300px');

    let accType = user.account_type;
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

            setTimeout(() => {
                success = false;
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

    function onWebauthnSuccess(data: WebauthnAdditionalData) {
        mfaPurpose = undefined;
        onSubmitFinish(data.code)
    }

    async function requestPasswordReset() {
        let res = await postPasswordResetRequest({
            email: user.email,
        });
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }
</script>

<div class="wrapper">
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
                <p>{t.account.federatedConvertPassword2}</p>
                {#if success}
                    <CheckIcon check/>
                {:else}
                    <Button level={3} onclick={requestPasswordReset}>
                        {t.account.passwordReset}
                    </Button>
                {/if}
            </div>
        {/if}

        {#if (accType === "passkey" || accType === "federated_passkey") && !convertAccount}
            <p>{t.account.accTypePasskeyText1}</p>
            <p>{t.account.accTypePasskeyText2}</p>
            <p>{t.account.accTypePasskeyText3}</p>
            <Button level={3} onclick={() => convertAccount = true}>
                {t.account.convertAccount}
            </Button>
        {/if}

        {#if accType === "password" || accType === "federated_password" || convertAccount}
            <div>
                <AccModPwd
                        bind:passwords
                        bind:isValid={isPwdValid}
                        inputWidth={inputWidth}
                        hideCurrentPassword={!(accType === "password" && passkeys.length < 1)}
                />

                <div class="save">
                    <Button onclick={onSubmit} level={1} {isLoading}>
                        {t.common.save}
                    </Button>
                </div>
                {#if convertAccount && !isLoading}
                    <div>
                        <Button level={3} onclick={() => convertAccount = false}>
                            {t.common.cancel}
                        </Button>
                    </div>
                {/if}
            </div>

            <div class="bottom">
                {#if success}
                    <div class="success" transition:fade>
                        <IconCheck/>
                    </div>
                {:else if err}
                    <div class="err" transition:fade>
                        {err}
                    </div>
                {/if}
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
</div>

<style>
    p {
        margin: .5rem 0;
    }

    .m-05 {
        margin: .5rem;
    }

    .wrapper {
        display: flex;
        flex-direction: row;
        overflow-x: clip;
    }

    .container {
        padding: 0 5px;
        display: flex;
        flex-direction: column;
    }

    .convertPasskey {
        margin: 1rem 0;
    }

    .bottom {
        height: 1em;
    }

    .save {
        margin-top: 1rem;
    }

    .success {
        margin: 5px;
        color: var(--col-ok);
    }
</style>
