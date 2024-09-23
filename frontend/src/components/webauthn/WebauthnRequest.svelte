<script>
    import {onMount} from "svelte";
    import {tweened} from "svelte/motion";
    import Loading from "$lib/Loading.svelte";
    import {webauthnAuth} from "../../utils/webauthn.js";
    import {promiseTimeout} from "../../utils/helpers.js";

    export let t = {
        invalidKeyUsed: 'Invalid Key',
        mfaAck: 'Acknowledged',
        provideMfa: 'Please login with your MFA device',
        requestExpires: 'Request expires',
    };
    export let data;
    export let purpose = 'Login';
    export let onError = (error) => {
    };
    export let onSuccess = (resBody) => {
    };
    let err = false;
    let msg = '';
    let success = false;

    let progress = tweened(data.exp, {
        duration: data.exp * 1000,
    })
    
    // close this component automatically, when the request has expired
    onMount(() => {
        let timer = setTimeout(() => {
            data = undefined;
        }, data.exp * 1000);
        progress.set(0);

        return () => clearTimeout(timer);
    });

    onMount(async () => {
        let p;
        if (purpose === 'Login') {
            p = {purpose: {Login: data.code}};
        } else {
            p = {purpose};
        }

        let res = {};
        try {
            res = await promiseTimeout(
                webauthnAuth(data.user_id, p, t.invalidKeyUsed),
                // we need to cancel 1 sec before the expiry to not get into a browser exception,
                // because we would not be able to "go back" again
                data.exp * 1000 - 1000
            );
        } catch (err) {
            console.error(err);
            res.err = true;
            res.msg = 'Timeout';
            onError('Passkey Error');
        }

        err = res.err;
        success = !res.err;
        msg = res.msg;

        if (success) {
            onSuccess(res.body);
        } else {
            setTimeout(() => {
                onError();
            }, 3000);
        }
    });

    //
</script>

<div class="wrapperOuter">
    <div class="wrapperInner">
        <div class="content">

            <div class="contentRow">
                <div class="contentHeader">
                    {t.provideMfa}
                </div>
            </div>

            <div class="contentRow">
                <div class="contentHeader">
                    {t.requestExpires}
                    :
                </div>
                <div>
                    <progress value={$progress} max={data.exp}></progress>
                </div>
            </div>

            <div class="contentRow">
                {#if success}
                    <div class="good">
                        {t.mfaAck}
                    </div>
                {:else if err}
                    <div class="err">
                        {msg}
                    </div>
                {:else}
                    <Loading background={false}/>
                {/if}
            </div>
        </div>

    </div>
</div>

<style>
    .content {
        width: 350px;
        height: 220px;
        border: 1px solid var(--col-ghigh);
        border-radius: 5px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: white;
        text-align: center;
        z-index: 20;
        background: rgba(24, 24, 24, .95);
    }

    .contentRow {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        margin: .25em;
    }

    .contentHeader {
        margin-bottom: 0.2em;
        font-weight: bold;
    }

    .err, .good {
        font-weight: bold;
    }

    .err {
        color: var(--col-err);
    }

    .good {
        color: var(--col-ok);
    }

    progress {
        accent-color: var(--col-acnt);
    }

    .wrapperOuter {
        position: absolute;
        top: 0;
        left: 0;
    }

    .wrapperInner {
        width: 100vw;
        height: 100vh;
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        background: rgba(0, 0, 0, .85);
        z-index: 20;
    }
</style>
