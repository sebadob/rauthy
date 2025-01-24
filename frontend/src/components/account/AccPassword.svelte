<script lang="ts">
    import Button from "$lib5/Button.svelte";
    import {blur, fade} from 'svelte/transition';
    import AccModPwd from "./AccModPwd.svelte";
    import {
        getUserPasskeys,
        postPasswordResetRequest,
        postUserSelfConvertPasskey,
    } from "$utils/dataFetching.js";
    import WebauthnRequest from "../webauthn/WebauthnRequest.svelte";
    import {onMount} from "svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {UpdateUserSelfRequest, UserResponse} from "$api/types/user.ts";
    import type {AuthProviderTemplate} from "$api/templates/AuthProvider.ts";
    import IconCheck from "$icons/IconCheck.svelte";
    import type {PropsPassword} from "./props.ts";
    import {fetchPut} from "$api/fetch.ts";
    import type {WebauthnAuthResult} from "../../webauthn/ceremony_auth.ts";
    import type {MfaPurpose} from "../../webauthn/types.ts";

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
    let passkeys = $state([]);
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
        let res = await getUserPasskeys(user.id);
        let body = await res.json();
        if (res.ok) {
            passkeys = body;
        } else {
            console.error('error fetching passkeys: ' + body.message);
        }
    }

    async function convertToPasskeyOnly() {
        let res = await postUserSelfConvertPasskey(user.id);
        if (res.ok) {
            window.location.reload();
        } else {
            let body = await res.json();
            console.error('error fetching passkeys: ' + body.message);
        }
    }

    async function onSubmit() {
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

        // TODO
        // const res = await webauthnAuthStart(user.id, {purpose: 'PasswordNew'});

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

    function onWebauthnError() {
        // If there is any error with the key, the user should start a new login process
        mfaPurpose = undefined;
        err = t.mfa.errorReg;
    }

    function onWebauthnSuccess(res: WebauthnAuthResult) {
        if (res) {
            mfaPurpose = undefined;
            console.warn('TODO onWebauthnSuccess()');
            // onSubmitFinish(res.data.code)
        }
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
                    <Button
                            onclick={requestPasswordReset}
                            level={3}
                    >
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
            <div in:blur={{ duration: 350 }}>
                <AccModPwd
                        bind:passwords
                        bind:isValid={isPwdValid}
                        inputWidth={inputWidth}
                        hideCurrentPassword={!(accType === "password" && passkeys.length < 1)}
                />

                <div>
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

            {#if !convertAccount && canConvertToPasskey}
                <div class="convertPasskey">
                    <h3>{t.account.convertAccount}</h3>
                    <p>{t.account.convertAccountP1}</p>
                    <Button level={3} onclick={convertToPasskeyOnly}>
                        {t.account.convertAccount}
                    </Button>
                </div>
            {/if}

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

    .success {
        margin: 5px;
        color: var(--col-ok);
    }
</style>
