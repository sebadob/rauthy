<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {onMount} from "svelte";
    import Input from "$lib5/form/Input.svelte";
    import {useI18n} from "$state/i18n.svelte.js";
    import {useSession} from "$state/session.svelte.js";
    import {fetchDelete, fetchGet, fetchPost} from "$api/fetch";
    import type {PasskeyResponse, WebauthnDeleteRequest} from "$api/types/webauthn.ts";
    import type {UserResponse} from "$api/types/user.ts";
    import {PATTERN_USER_NAME} from "$utils/patterns";
    import {webauthnReg} from "$webauthn/registration";
    import WebauthnRequest from "$lib5/WebauthnRequest.svelte";
    import type {MfaPurpose, WebauthnAdditionalData, WebauthnServiceReq} from "$webauthn/types.ts";
    import UserPasskey from "$lib5/UserPasskey.svelte";
    import type {MfaModTokenResponse, UserMfaTokenRequest} from "$api/types/mfa_mod_token";
    import Modal from "$lib/Modal.svelte";
    import InputPassword from "$lib/form/InputPassword.svelte";
    import Form from "$lib/form/Form.svelte";
    import IconArrowPathSquare from "$icons/IconArrowPathSquare.svelte";

    let {user}: { user: UserResponse } = $props();

    const isSupported = 'credentials' in navigator;

    let t = useI18n();
    let session = useSession('account');
    let userId = $derived(session.get()?.user_id);

    let refInput: undefined | HTMLInputElement = $state();
    let refPkAuthBtn: undefined | HTMLButtonElement = $state();

    let err = $state(false);
    let pwdErr = $state('');
    let msg = $state('');
    let showRegInput = $state(false);
    let showDelete = $state(user.account_type === "password");

    let mfaPurpose: undefined | MfaPurpose = $state();
    let passkeyName = $state('');
    let isInputError = $state(false);
    let isLoading = $state(false);

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let passkeys: PasskeyResponse[] = $state([]);
    let mfaModToken: undefined | MfaModTokenResponse = $state();
    let mfaModSecs: undefined | number = $state();
    let interval: undefined | number;

    onMount(() => {
        fetchPasskeys();
    });

    $effect(() => {
        if (passkeys.length > 0 && user.account_type === "passkey") {
            showDelete = passkeys.length > 1;
        }
    });

    $effect(() => {
        refInput?.focus();
    });

    $effect(() => {
        refPkAuthBtn?.focus();
    });

    function calcModSecs() {
        let modSecs = 0;
        if (mfaModToken) {
            let ts = new Date().getTime() / 1000;
            modSecs = Math.floor(mfaModToken.exp - ts);
        }
        if (modSecs > 0) {
            mfaModSecs = modSecs;
        } else {
            mfaModSecs = undefined;
            mfaModToken = undefined;
            clearInterval(interval);
            interval = undefined;
        }
    }

    function resetMsgErr() {
        err = false;
        msg = '';
    }

    async function fetchPasskeys() {
        err = false;

        let res = await fetchGet<PasskeyResponse[]>(`/auth/v1/users/${session.get()?.user_id}/webauthn`);
        if (res.body) {
            passkeys = res.body;
        } else {
            err = true;
        }
    }

    async function handleRegister() {
        resetMsgErr();

        if (isInputError || !userId) {
            return;
        }
        if (passkeyName.length < 1) {
            err = true;
            msg = t.mfa.passkeyNameErr;
            return;
        }

        let tokenId = mfaModToken?.id;
        if (!tokenId) {
            showModal = true;
            return;
        }

        let res = await webauthnReg(
            userId,
            passkeyName,
            t.authorize.invalidKeyUsed,
            t.authorize.requestExpired,
            undefined,
            undefined,
            tokenId,
        );
        if (res.error) {
            err = true;
            msg = `${t.mfa.errorReg} - ${res.error}`;
        } else {
            showRegInput = false;
            passkeyName = '';
            await fetchPasskeys();
        }
    }

    async function handleDelete(name: string) {
        if (!mfaModToken) {
            showModal = true;
            return;
        }

        let payload: WebauthnDeleteRequest = {
            mfa_mod_token_id: mfaModToken.id,
        };
        let res = await fetchDelete(`/auth/v1/users/${user.id}/webauthn/delete/${name}`, payload);
        if (res.status === 200) {
            await fetchPasskeys();
        } else {
            msg = res.error?.message || 'Error';
        }
    }

    function onRegisterClick() {
        if (mfaModToken) {
            showRegInput = true;
        } else {
            showModal = true;
        }
    }

    async function onMfaTokenSubmit(_form: HTMLFormElement, params: URLSearchParams) {
        pwdErr = '';
        isLoading = true;

        let payload: UserMfaTokenRequest = {
            password: params.get("password") || '',
        }
        await fetchMfaToken(payload);
        isLoading = false;
    }

    async function fetchMfaToken(payload: UserMfaTokenRequest) {
        let res = await fetchPost<MfaModTokenResponse>(`/auth/v1/users/${user.id}/mfa_token`, payload);
        if (res.body) {
            mfaModToken = res.body;
            closeModal?.();

            if (interval) {
                clearInterval(interval);
            }

            calcModSecs();
            interval = setInterval(() => {
                calcModSecs()
            }, 1000);
        } else {
            pwdErr = t.mfa.passwordInvalid;
        }
    }

    function mfaTokenRefresh() {
        mfaModToken = undefined;
        showModal = true;
    }

    async function onMfaTokenWebauthnSubmit() {
        closeModal?.();
        mfaPurpose = 'MfaModToken';
    }

    function onWebauthnError(error: string) {
        mfaPurpose = undefined;
        err = true;
        msg = error;
        setTimeout(() => {
            err = false;
            msg = '';
        }, 5000);
    }

    function onWebauthnSuccess(data?: WebauthnAdditionalData) {
        if (mfaPurpose === 'MfaModToken') {
            if (!data) {
                console.error('did not receive WebauthnData after SvcReq');
                return;
            }
            let svc = data as WebauthnServiceReq;
            let payload: UserMfaTokenRequest = {
                mfa_code: svc.code,
            }
            fetchMfaToken(payload);
        } else {
            msg = t.mfa.testSuccess;
            setTimeout(() => {
                msg = '';
            }, 3000);
        }

        mfaPurpose = undefined;
    }
</script>

<div class="container">
    {#if !isSupported}
        <div class="err">
            <b>
                Your browser does not support Webauthn credentials and must be updated.
            </b>
        </div>
    {:else}
        {#if mfaPurpose}
            <WebauthnRequest
                    userId={user.id}
                    purpose={mfaPurpose}
                    onSuccess={onWebauthnSuccess}
                    onError={onWebauthnError}
            />
        {/if}

        <p>
            {t.mfa.p1}
            <br><br>
            {t.mfa.p2}
        </p>

        {#if mfaModSecs && mfaModSecs > 0}
            <div class="modToken">
                <div>
                    {t.account.canModifyFor}
                    <span class="timeLeft">
                        {mfaModSecs}
                        {t.common.seconds}
                    </span>
                </div>
                <Button ariaLabel={t.common.refresh} invisible onclick={mfaTokenRefresh}>
                    <div class="btnRefresh">
                        <IconArrowPathSquare/>
                    </div>
                </Button>
            </div>
        {/if}

        {#if showRegInput}
            <Input
                    bind:ref={refInput}
                    bind:value={passkeyName}
                    autocomplete="off"
                    label={t.mfa.passkeyName}
                    placeholder={t.mfa.passkeyName}
                    maxLength={32}
                    pattern={PATTERN_USER_NAME}
                    bind:isError={isInputError}
                    onEnter={handleRegister}
            />
            <div class="regBtns">
                <Button onclick={handleRegister}>{t.mfa.register}</Button>
                <Button level={3} onclick={() => showRegInput = false}>{t.common.cancel}</Button>
            </div>
        {:else}
            <div class="regNewBtn">
                <Button
                        level={passkeys.length === 0 ? 1 : 2}
                        onclick={onRegisterClick}
                >
                    {t.mfa.registerNew}
                </Button>
                <Modal bind:showModal bind:closeModal>
                    {#if user.webauthn_user_id}
                        <p style:max-width="20rem">
                            {t.mfa.reAuthenticatePasskey}
                        </p>
                        <ul>
                            {#each passkeys as pk}
                                <li>{pk.name}</li>
                            {/each}
                        </ul>

                        <div style:margin-top="1rem">
                            <Button bind:ref={refPkAuthBtn} onclick={onMfaTokenWebauthnSubmit}>
                                {t.common.authenticate}
                            </Button>
                        </div>
                    {:else}
                        <p style:max-width="20rem">
                            {t.mfa.reAuthenticatePwd}
                        </p>

                        <Form action="" onSubmit={onMfaTokenSubmit}>
                            <InputPassword
                                    bind:ref={refInput}
                                    name="password"
                                    autocomplete="current-password"
                                    label={t.account.passwordCurr}
                                    placeholder={t.account.passwordCurr}
                                    required
                            />
                            <Button type="submit" {isLoading}>{t.common.authenticate}</Button>
                            {#if pwdErr}
                                <div class="pwdInvalid">
                                    {pwdErr}
                                </div>
                            {/if}
                        </Form>
                    {/if}
                </Modal>
            </div>
        {/if}

        {#if passkeys.length > 0}
            <div class="keysHeader">
                {t.mfa.registerdKeys}
            </div>
        {/if}
        <div class="keysContainer">
            {#each passkeys as passkey (passkey.name)}
                <UserPasskey {passkey} {showDelete} onDelete={handleDelete}/>
            {/each}
        </div>

        {#if passkeys.length > 0}
            <div class="button">
                <Button onclick={() => mfaPurpose = 'Test' }>{t.mfa.test}</Button>
            </div>
        {/if}

        <div class:success={!err} class:err>
            {msg}
        </div>
    {/if}
</div>

<style>
    p {
        margin: .5rem 0;
    }

    .btnRefresh {
        color: hsla(var(--text) / .5);
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .button {
        margin-top: .33rem;
    }

    .keysContainer {
        width: 100%;
        max-height: 20rem;
        overflow-y: auto;
    }

    .keysHeader {
        margin-top: .5rem;
        font-weight: bold;
    }

    .modToken {
        display: flex;
        gap: .5rem;
    }

    .pwdInvalid {
        color: hsl(var(--error));
        margin: .5rem 0;
    }

    .success, .err {
        margin: .5rem -.3rem;
        text-align: left;
    }

    .success {
        margin-left: .2rem;
        color: hsl(var(--action));
    }

    .timeLeft {
        color: hsl(var(--action));
    }

    .regBtns {
        margin: .25rem 0;
        display: flex;
        align-items: center;
        gap: .5rem;
    }

    .regNewBtn {
        margin: .5rem 0;
    }
</style>
