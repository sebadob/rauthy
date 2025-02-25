<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {onMount} from "svelte";
    import Input from "$lib5/form/Input.svelte";
    import {useI18n} from "$state/i18n.svelte.js";
    import {useSession} from "$state/session.svelte.js";
    import {fetchDelete, fetchGet} from "$api/fetch.ts";
    import type {PasskeyResponse} from "$api/types/webauthn.ts";
    import type {UserResponse} from "$api/types/user.ts";
    import {PATTERN_USER_NAME} from "$utils/patterns.ts";
    import {webauthnReg} from "$webauthn/registration.ts";
    import WebauthnRequest from "$lib5/WebauthnRequest.svelte";
    import type {MfaPurpose, WebauthnAdditionalData} from "$webauthn/types.ts";
    import UserPasskey from "$lib5/UserPasskey.svelte";

    let {user}: { user: UserResponse } = $props();

    const isSupported = 'credentials' in navigator;

    let t = useI18n();
    let session = useSession('account');
    let userId = $derived(session.get()?.user_id);

    let refInput: undefined | HTMLInputElement = $state();

    let err = $state(false);
    let msg = $state('');
    let showRegInput = $state(false);
    let showDelete = $state(user.account_type === "password");

    let mfaPurpose: undefined | MfaPurpose = $state();
    let passkeyName = $state('');
    let isInputError = $state(false);

    let passkeys: PasskeyResponse[] = $state([]);

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

    function resetMsgErr() {
        err = false;
        msg = '';
    }

    async function fetchPasskeys() {
        let res = await fetchGet<PasskeyResponse[]>(`/auth/v1/users/${session.get()?.user_id}/webauthn`);
        if (res.body) {
            passkeys = res.body;
        } else {
            console.error(res.error);
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

        let res = await webauthnReg(userId, passkeyName, t.authorize.invalidKeyUsed, t.authorize.requestExpired);
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
        let res = await fetchDelete(`/auth/v1/users/${user.id}/webauthn/delete/${name}`);
        if (res.status === 200) {
            await fetchPasskeys();
        } else {
            msg = res.error?.message || 'Error';
        }
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
        mfaPurpose = undefined;
        msg = t.mfa.testSuccess;
        setTimeout(() => {
            msg = '';
        }, 3000);
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
                        onclick={() => showRegInput = true}
                >
                    {t.mfa.registerNew}
                </Button>
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
        max-height: 20rem;
        overflow-y: auto;
    }

    .keysHeader {
        margin-top: .5rem;
        font-weight: bold;
    }

    .success, .err {
        margin: .5rem -.3rem;
        text-align: left;
    }

    .success {
        margin-left: .2rem;
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
