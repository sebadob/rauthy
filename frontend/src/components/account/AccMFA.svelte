<script lang="ts">
    import {formatDateFromTs} from "$utils/helpers";
    import Button from "$lib5/Button.svelte";
    import {webauthnDelete} from "$utils/dataFetching.js";
    import {onMount} from "svelte";
    import Input from "$lib5/form/Input.svelte";
    import IconFingerprint from "$lib/icons/IconFingerprint.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useSession} from "$state/session.svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {PasskeyResponse} from "$api/types/webauthn.ts";
    import type {UserResponse} from "$api/types/user.ts";
    import {PATTERN_USER_NAME} from "$utils/patterns.ts";
    import {webauthnReg} from "$webauthn/ceremony_reg.ts";
    import {webauthnAuth} from "$webauthn/ceremony_auth.ts";

    let {user}: { user: UserResponse } = $props();

    let t = useI18n();
    let session = useSession();
    let userId = $derived(session.get()?.user_id);

    let refInput: undefined | HTMLInputElement = $state();

    let err = $state(false);
    let msg = $state('');
    let showRegInput = $state(false);
    let showDelete = $state(user.account_type === "password");

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

    async function handleRegStart() {
        resetMsgErr();

        if (isInputError || !userId) {
            return;
        }
        if (passkeyName.length < 1) {
            err = true;
            msg = t.mfa.passkeyNameErr;
            return;
        }

        let res = await webauthnReg(userId, passkeyName);
        if (res.success) {
            showRegInput = false;
            passkeyName = '';
            await fetchPasskeys();
        } else {
            err = true;
            msg = `${t.mfa.errorReg} - ${res.msg}`;
        }
    }

    async function handleTestStart() {
        resetMsgErr();

        let res = await webauthnAuth(user.id, 'Test', t.mfa.testError);
        if (res.success) {
            msg = t.mfa.testSuccess;
            // re-fetching keys to update timestamps for expected outcome
            await fetchPasskeys();
        } else {
            err = true;
            msg = `${t.mfa.testError} - ${res.msg}`;
        }
    }

    async function handleDelete(name: string) {
        let res = await webauthnDelete(user.id, name);
        if (res.status === 200) {
            await fetchPasskeys();
        } else {
            let body = await res.json();
            err = true;
            msg = body.message;
        }
    }

</script>

<div class="container">
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
                onEnter={handleRegStart}
        />
        <div class="regBtns">
            <Button onclick={handleRegStart}>{t.mfa.register}</Button>
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
            <div class="keyContainer">
                <div class="row">
                    {`${t.mfa.passkeyName}: `}
                    <div class="nameUv">
                        <b>{passkey.name}</b>
                        {#if passkey.user_verified}
                            <Tooltip text={t.account.userVerifiedTooltip}>
                                <div style:margin-bottom="-.25rem">
                                    <IconFingerprint width={18} color="var(--col-acnt)"/>
                                </div>
                            </Tooltip>
                        {/if}
                    </div>
                </div>
                <div class="row">
                    {`${t.mfa.registerd}: `}
                    <span class="font-mono">{formatDateFromTs(passkey.registered)}</span>
                </div>
                <div class="row">
                    {`${t.mfa.lastUsed}: `}
                    <span class="font-mono">{formatDateFromTs(passkey.last_used)}</span>
                </div>

                {#if showDelete}
                    <div class="row">
                        <div></div>
                        <div class="deleteBtn">
                            <Button
                                    level={-3}
                                    onclick={() => handleDelete(passkey.name)}
                            >
                                {t.common.delete.toUpperCase()}
                            </Button>
                        </div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>

    {#if passkeys.length > 0}
        <div class="button">
            <Button onclick={handleTestStart}>{t.mfa.test}</Button>
        </div>
    {/if}

    <div class:success={!err} class:err>
        {msg}
    </div>
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
        margin-left: -.33rem;
    }

    .deleteBtn {
        margin-right: -.35rem;
    }

    .keysContainer {
        max-height: 20rem;
        padding-right: 2rem;
        overflow-y: auto;
    }

    .keyContainer {
        margin: .33rem 0;
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
        color: hsl(var(--action));
    }

    .nameUv {
        display: inline-flex;
        align-items: center;
        gap: .25rem;
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

    .row {
        display: flex;
        gap: .5rem;
        justify-content: space-between;
        align-items: center;
    }
</style>
