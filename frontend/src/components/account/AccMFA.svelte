<script>
    import {arrBufToBase64UrlSafe, base64UrlSafeToArrBuf, redirectToLogin} from "../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {
        getUser,
        webauthnAuthFinish,
        webauthnAuthStart,
        webauthnDelete,
        webauthnRegFinish,
        webauthnRegStart
    } from "../../utils/dataFetching.js";
    import {sleepAwait} from "$lib/utils/helpers.js";

    export let t;
    export let sessionInfo;
    export let user = {};

    let isLoading = false;
    let err = false;
    let msg = '';

    function resetMsgErr() {
        err = false;
        msg = '';
    }

    async function fetchUpdatedUser() {
        resetMsgErr();

        let res = await getUser(sessionInfo.user_id);
        if (res.ok) {
            user = await res.json();
        } else {
            redirectToLogin('account');
        }
    }

    async function handleRegStart(slot) {
        resetMsgErr();

        let res = await webauthnRegStart(user.id, {slot});
        if (res.status === 200) {
            let challenge = await res.json();

            // the navigator credentials engine needs some values as array buffers
            challenge.publicKey.challenge = base64UrlSafeToArrBuf(challenge.publicKey.challenge);
            challenge.publicKey.user.id = base64UrlSafeToArrBuf(challenge.publicKey.user.id);

            // prompt for the user security key and get its public key
            let challengePk = await navigator.credentials.create(challenge);

            // the backend expects base64 url safe string instead of array buffers
            let data = {
                slot,
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
                await fetchUpdatedUser();
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
            } else {
                console.error(res);
            }
        } else {
            err = true;
            msg = t.mfa.testError;
        }
    }

    async function handleDeleteSlot(slot) {
        let res = await webauthnDelete(user.id, slot);
        if (res.status === 200) {
            await fetchUpdatedUser();
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
        <br><br>
        {t.mfa.p3}
    </p>

    <div class="keyContainer">
        {#if user.sec_key_1}
            <div>
                <div class="key">
                    {t.mfa.securityKey} 1:
                </div>
                <div class="id">
                    {user.sec_key_1}
                </div>
            </div>

            <div class="btn">
                <Button on:click={() => handleDeleteSlot(1)} level={3}>{t.mfa.delete.toUpperCase()}</Button>
            </div>
        {:else}
            <div>
                {t.mfa.noKey}
            </div>
            <div class="btn">
                <Button on:click={() => handleRegStart(1)}>{t.mfa.register.toUpperCase()}</Button>
            </div>
        {/if}
    </div>

    <div style="height: 20px"></div>

    <div class="keyContainer">
        {#if user.sec_key_2}
            <div>
                <div class="key">
                    {t.mfa.securityKey} 2:
                </div>
                <div class="id">
                    {user.sec_key_2}
                </div>
            </div>

            <div class="btn">
                <Button on:click={() => handleDeleteSlot(2)} level={3}>{t.mfa.delete.toUpperCase()}</Button>
            </div>
        {:else}
            <div>
                {t.mfa.noKey}
            </div>
            <div class="btn">
                <Button on:click={() => handleRegStart(2)}>{t.mfa.register.toUpperCase()}</Button>
            </div>
        {/if}
    </div>

    {#if user.sec_key_1 || user.sec_key_2}
        <div class="button">
            <Button on:click={handleTestStart} level={1}>{t.mfa.test.toUpperCase()}</Button>
        </div>
    {/if}

    <div class:msg={!err} class:err>
        {msg}
    </div>
</div>

<style>
    .btn {
        margin-left: -5px;
    }

    .container {
        margin-top: -10px;
        padding-left: 10px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .button {
        padding: 15px 0 0 5px;
    }

    .keyContainer {
        width: 250px;
        padding: 10px;
        border: 1px solid var(--col-gmid);
    }

    .key {
        font-weight: bold;
    }

    .id {
        margin-bottom: 5px;
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
</style>
