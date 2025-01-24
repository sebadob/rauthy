<script lang="ts">
    import {onMount} from "svelte";
    import Loading from "$lib/Loading.svelte";
    import {webauthnAuth, type WebauthnAuthResult} from "$webauthn/ceremony_auth.ts";
    import {promiseTimeout} from "$utils/helpers";
    import {useI18n} from "$state/i18n.svelte";
    import type {MfaPurpose} from "$webauthn/types.ts";

    let {
        userId,
        purpose,
        onError,
        onSuccess,
    }: {
        userId: string,
        purpose: MfaPurpose,
        onError: (error: string) => void,
        onSuccess: (res: WebauthnAuthResult) => void,
    } = $props();

    let t = useI18n();

    let err = $state(false);
    let msg = $state('');
    let success = $state(false);

    let exp: undefined | number = $state();
    let progress = $state(100);

    $effect(() => {
        let timer: number;
        if (exp) {
            timer = setTimeout(() => {
                onError('Timeout');
            }, exp * 1000);
        }

        return () => clearTimeout(timer);
    });

    onMount(async () => {
        let res = {};
        try {
            res = await promiseTimeout(
                webauthnAuth(userId, purpose, t.authorize.invalidKeyUsed),
                // we need to cancel 1 sec before the expiry to not get into a browser exception,
                // because we would not be able to "go back" again
                exp * 1000 - 1000
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

{#if purpose}
    <div class="wrapperOuter">
        <div class="wrapperInner">
            <div class="content">
                <div class="contentRow">
                    <div class="contentHeader">
                        {t.authorize.provideMfa}
                    </div>
                </div>

                <div class="contentRow">
                    <div class="contentHeader">
                        {t.authorize.requestExpires}
                        :
                    </div>
                    <div>
                        <progress value={progress} max={exp}></progress>
                    </div>
                </div>

                <div class="contentRow">
                    {#if success}
                        <div class="good">
                            {t.authorize.mfaAck}
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
{/if}

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
