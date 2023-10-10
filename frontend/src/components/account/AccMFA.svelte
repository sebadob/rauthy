<script>
    import {
        arrBufToBase64UrlSafe,
        base64UrlSafeToArrBuf,
        extractFormErrors,
        formatDateFromTs
    } from "../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {
        getUserPasskeys,
        webauthnAuthFinish,
        webauthnAuthStart,
        webauthnDelete,
        webauthnRegFinish,
        webauthnRegStart
    } from "../../utils/dataFetching.js";
    import {onMount} from "svelte";
    import Input from "$lib/inputs/Input.svelte";
    import * as yup from "yup";
    import {REGEX_NAME} from "../../utils/constants.js";
    import IconFingerprint from "$lib/icons/IconFingerprint.svelte";
    import Tooltip from "$lib/Tooltip.svelte";

    export let t;
    export let sessionInfo;
    export let user = {};

    let isLoading = false;
    let err = false;
    let msg = '';
    let showRegInput = false;
    let showDelete = user.account_type === "password";

    let passkeys = [];
    $: if (passkeys.length > 0 && user.account_type === "passkey") {
        showDelete = passkeys.length > 1;
    }

    let formValues = {passkeyName: ''};
    let formErrors = {};
    const schema = yup.object().shape({
        passkeyName: yup.string()
            .required(t.invalidInput)
            .matches(REGEX_NAME, t.mfa.passkeyNameErr),
    });

    onMount(() => {
        fetchPasskeys();
    });

    function resetMsgErr() {
        err = false;
        msg = '';
    }

    async function fetchPasskeys() {
        let res = await getUserPasskeys(sessionInfo.user_id);
        let body = await res.json();
        if (res.ok) {
            passkeys = body;
        } else {
            console.error('error fetching passkeys: ' + body.message);
        }
    }

    async function handleRegStart() {
        resetMsgErr();

        const valid = await validateForm();
        if (!valid) {
            return;
        }

        const passkeyName = formValues.passkeyName;
        if (passkeyName.length < 2) {
            err = true;
            msg = t.mfa.passkeyNameErr;
            return;
        }

        let res = await webauthnRegStart(user.id, {passkey_name: passkeyName});
        if (res.status === 200) {
            let challenge = await res.json();

            // the navigator credentials engine needs some values as array buffers
            challenge.publicKey.challenge = base64UrlSafeToArrBuf(challenge.publicKey.challenge);
            challenge.publicKey.user.id = base64UrlSafeToArrBuf(challenge.publicKey.user.id);
            challenge.publicKey.excludeCredentials = challenge.publicKey.excludeCredentials

            if (challenge.publicKey.excludeCredentials) {
                challenge.publicKey.excludeCredentials = challenge.publicKey.excludeCredentials.map(cred => {
                    cred.id = base64UrlSafeToArrBuf(cred.id);
                    return cred;
                });
            }

            // prompt for the user security key and get its public key
            let challengePk = await navigator.credentials.create(challenge);

            // the backend expects base64 url safe string instead of array buffers
            let data = {
                passkey_name: passkeyName,
                data: {
                    id: challengePk.id,
                    rawId: arrBufToBase64UrlSafe(challengePk.rawId),
                    response: {
                        attestationObject: arrBufToBase64UrlSafe(challengePk.response.attestationObject),
                        clientDataJSON: arrBufToBase64UrlSafe(challengePk.response.clientDataJSON),
                    },
                    type: challengePk.type,
                },
            }

            // send the keys' pk to the backend and finish the registration
            res = await webauthnRegFinish(user.id, data);
            if (res.status === 201) {
                showRegInput = false;
                formValues.passkeyName = '';
                await fetchPasskeys();
            } else {
                console.error(res);
            }
        } else {
            err = true;
            msg = t.mfa.errorReg;
        }
    }

    async function handleTestStart() {
        resetMsgErr();

        let res = await webauthnAuthStart(user.id, {purpose: 'Test'});
        if (res.status === 200) {
            let resp = await res.json();
            let challenge = resp.rcr;

            // the navigator credentials engine needs some values as array buffers
            challenge.publicKey.challenge = base64UrlSafeToArrBuf(challenge.publicKey.challenge);
            for (let cred of challenge.publicKey.allowCredentials) {
                cred.id = base64UrlSafeToArrBuf(cred.id);
            }

            // prompt for the user security key and get its public key
            let challengePk
            try {
                challengePk = await navigator.credentials.get(challenge);
            } catch (e) {
                err = true;
                msg = t.mfa.invalidKeyUsed;
                return;
            }

            // the backend expects base64 url safe string instead of array buffers
            let data = {
                code: resp.code,
                data: {
                    id: challengePk.id,
                    rawId: arrBufToBase64UrlSafe(challengePk.rawId),
                    response: {
                        authenticatorData: arrBufToBase64UrlSafe(challengePk.response.authenticatorData),
                        clientDataJSON: arrBufToBase64UrlSafe(challengePk.response.clientDataJSON),
                        signature: arrBufToBase64UrlSafe(challengePk.response.signature),
                    },
                    type: challengePk.type,
                }
            }

            // send the data to the backend
            res = await webauthnAuthFinish(user.id, data);
            if (res.status === 202) {
                msg = t.mfa.testSuccess;
                await fetchPasskeys();
            } else {
                console.error(res);
            }
        } else {
            err = true;
            msg = t.mfa.testError;
        }
    }

    async function handleDelete(name) {
        let res = await webauthnDelete(user.id, name);
        if (res.status === 200) {
            await fetchPasskeys();
        } else {
            let body = await res.json();
            err = true;
            msg = body.message;
        }
    }

    async function validateForm() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
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
                bind:value={formValues.passkeyName}
                bind:error={formErrors.passkeyName}
                autocomplete="off"
                placeholder={t.mfa.passkeyName}
                on:input={validateForm}
                autofocus
                on:enter={handleRegStart}
        >
            {t.mfa.passkeyName}
        </Input>
        <div class="regBtns">
            <Button on:click={handleRegStart} level={1}>{t.mfa.register.toUpperCase()}</Button>
            <Button on:click={() => showRegInput = false} level={4}>{t.cancel.toUpperCase()}</Button>
        </div>
    {:else}
        <div class="regNewBtn">
            <Button on:click={() => showRegInput = true} level={3}>{t.mfa.registerNew.toUpperCase()}</Button>
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
                            <Tooltip text={t.userVerifiedTooltip}>
                                <div style:margin-bottom="-.25rem">
                                    <IconFingerprint width=18 color="var(--col-acnt)" />
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
                                    on:click={() => handleDelete(passkey.name)}
                                    level={4}
                                    disabled={showDelete}
                            >
                                {t.mfa.delete.toUpperCase()}
                            </Button>
                        </div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>

    {#if passkeys.length > 0}
        <div class="button">
            <Button on:click={handleTestStart} level={1}>{t.mfa.test.toUpperCase()}</Button>
        </div>
    {/if}

    <div class:msg={!err} class:err>
        {msg}
    </div>
</div>

<style>
    p {
        margin: .5rem 0;
    }

    .container {
        padding-left: 10px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .button {
        margin-left: -.33rem;
    }

    .deleteBtn {
        margin-right: -.8rem;
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

    .msg, .err {
        margin: 5px;
        text-align: center;
    }

    .err {
        color: var(--col-err);
    }

    .msg {
        color: var(--col-ok);
    }

    .nameUv {
        display: inline-flex;
        align-items: center;
        gap: .25rem;
    }

    .regBtns {
        display: flex;
        align-items: center;
    }

    .regNewBtn {
        margin: 0 0 .5rem -.33rem;
    }

    .row {
        display: flex;
        gap: .5rem;
        justify-content: space-between;
        align-items: center;
    }
</style>
