<script>
    import Button from "$lib/Button.svelte";
    import {blur, fade} from 'svelte/transition';
    import AccModPwd from "./AccModPwd.svelte";
    import {
        getUserPasskeys, postPasswordResetRequest,
        postUserSelfConvertPasskey,
        putUserSelf,
        webauthnAuthStart
    } from "../../utils/dataFetching.js";
    import WebauthnRequest from "../webauthn/WebauthnRequest.svelte";
    import {onMount} from "svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";

    export let t;
    export let user = {};
    export let authProvider;
    export let viewModePhone = false;
    $: inputWidth = viewModePhone ? 'calc(100vw - 1.5rem)' : '300px';

    const btnWidth = "12rem";

    let accType = user.account_type;
    let passkeys = [];
    let isPwdValid;
    let convertAccount = false;
    let isLoading = false;
    let err = '';
    let success = false;
    let webauthnData;

    let formValues = {};
    let formErrors = {};

    $: canConvertToPasskey = passkeys.filter(pk => pk.user_verified).length > 0;

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
        const validPwd = await isPwdValid();
        if (!validPwd) {
            err = t.invalidInput;
            return;
        }

        const res = await webauthnAuthStart(user.id, {purpose: 'PasswordNew'});
        webauthnData = await res.json();
    }

    async function onSubmitFinish(mfaCode) {
        const validPwd = await isPwdValid();
        if (!validPwd) {
            err = t.invalidInput;
            return;
        }

        isLoading = true;

        let data = {
            password_new: formValues.new,
        };
        if (mfaCode) {
            data.mfa_code = mfaCode;
        } else {
            data.password_current = formValues.current;
        }

        let res = await putUserSelf(user.id, data);
        if (res.ok) {
            success = true;
            formValues = {};
            window.location.reload();
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    function onWebauthnError() {
        // If there is any error with the key, the user should start a new login process
        webauthnData = undefined;
        err = t.mfa.errorReg;
    }

    function onWebauthnSuccess(res) {
        if (res) {
            webauthnData = undefined;
            onSubmitFinish(res.code)
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
        {#if webauthnData}
            <WebauthnRequest
                    bind:data={webauthnData}
                    purpose="PasswordNew"
                    onSuccess={onWebauthnSuccess}
                    onError={onWebauthnError}
            />
        {/if}

        {#if accType === 'federated'}
            <div class="m-05">
                <p>{t.federatedConvertPassword1}</p>
                <p><b>{authProvider.name || 'UNKNOWN'}</b></p>
                <p>{t.federatedConvertPassword2}</p>
                {#if success}
                    <CheckIcon check/>
                {:else}
                    <Button
                            width={btnWidth}
                            on:click={requestPasswordReset}
                            level={3}
                    >
                        {t.passwordReset.toUpperCase()}
                    </Button>
                {/if}
            </div>
        {/if}

        {#if (accType === "passkey" || accType === "federated_passkey") && !convertAccount}
            <p>{t.accTypePasskeyText1}</p>
            <p>{t.accTypePasskeyText2}</p>
            <p>{t.accTypePasskeyText3}</p>
            <Button
                    width={btnWidth}
                    on:click={() => convertAccount = true}
                    level={3}
            >
                {t.convertAccount.toUpperCase()}
            </Button>
        {/if}

        {#if accType === "password" || accType === "federated_password" || convertAccount}
            <div in:blur={{ duration: 350 }}>
                <AccModPwd
                        bind:t
                        bind:formValues
                        bind:isValid={isPwdValid}
                        btnWidth={btnWidth}
                        inputWidth={inputWidth}
                        hideCurrentPassword={!(accType === "password" && passkeys.length < 1)}
                />

                <div>
                    <Button width={btnWidth} on:click={onSubmit} level={1} bind:isLoading>
                        {t.save.toUpperCase()}
                    </Button>
                </div>
                {#if convertAccount && !isLoading}
                    <div>
                        <Button
                                width={btnWidth}
                                on:click={() => convertAccount = false}
                                level={4}
                        >
                            {t.cancel.toUpperCase()}
                        </Button>
                    </div>
                {/if}
            </div>

            {#if !convertAccount && canConvertToPasskey}
                <div class="convertPasskey">
                    <h3>{t.convertAccount}</h3>
                    <p>{t.convertAccountP1}</p>
                    <Button
                            width={btnWidth}
                            on:click={convertToPasskeyOnly}
                            level={3}
                    >
                        {t.convertAccount.toUpperCase()}
                    </Button>
                </div>
            {/if}

            <div class="bottom">
                {#if success}
                    <div class="success" transition:fade>
                        Update successful
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

    .err {
        width: 230px;
        margin: -5px 10px 0 35px;
        padding-right: 5px;
        color: var(--col-err);
    }

    .err {
        margin: 5px;
        color: var(--col-err);
    }

    .bottom {
        height: 1em;
    }

    .success {
        margin: 5px;
        color: var(--col-ok);
    }
</style>
