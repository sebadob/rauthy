<script lang="ts">
    import Loading from '$lib5/Loading.svelte';
    import { webauthnAuth, type WebauthnAuthResult } from '$webauthn/authentication';
    import { useI18n } from '$state/i18n.svelte.js';
    import type { MfaPurpose, WebauthnAdditionalData } from '$webauthn/types.ts';
    import { onMount } from 'svelte';

    let {
        userId,
        purpose,
        onError,
        onSuccess,
    }: {
        userId: string;
        purpose: MfaPurpose;
        onError: (error: string) => void;
        onSuccess: (res?: WebauthnAdditionalData) => void;
    } = $props();

    let t = useI18n();

    let webauthnRes: undefined | WebauthnAuthResult = $state();

    onMount(async () => {
        webauthnRes = await webauthnAuth(userId, purpose, t.authorize.invalidKeyUsed, t.authorize.requestExpired);
    });

    $effect(() => {
        if (webauthnRes) {
            if (webauthnRes.error) {
                setTimeout(() => {
                    onError(webauthnRes?.error || 'Webauthn Error');
                }, 3000);
            } else {
                onSuccess(webauthnRes.data);
            }
        }
    });
</script>

{#if purpose}
    <div class="wrapperOuter">
        <div class="wrapperInner">
            <div class="content">
                <div class="contentRow">
                    <div class="contentHeader">
                        {t.authorize.expectingPasskey}
                    </div>
                </div>

                <div class="contentRow">
                    <!-- <div class="contentHeader muted">-->
                    <!--     {t.authorize.requestExpires}-->
                    <!--     :-->
                    <!-- </div>-->
                    <div>
                        <!-- TODO
                        We can't show an expiry visualization based on JS because an open PIN / key
                        request window will block the JS event loop -> Find a way to show the timeout
                        with CSS only before it gets blocked.
                        Stick with just loading until then.
                        -->
                        {#if !webauthnRes}
                            <Loading />
                        {/if}
                        <!-- <progress value={progress} max={exp}></progress>-->
                    </div>
                </div>

                <div class="contentRow">
                    {#if webauthnRes}
                        {#if webauthnRes.error}
                            <div class="err">
                                {webauthnRes.error}
                            </div>
                        {:else}
                            <div class="good">
                                {t.authorize.mfaAck}
                            </div>
                        {/if}
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .content {
        padding: 1rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: hsl(var(--text-high));
        text-align: center;
        z-index: 99;
        background: hsla(var(--bg) / 0.9);
    }

    .contentRow {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        margin: 0.25em;
    }

    .contentHeader {
        margin-bottom: 0.2em;
        font-weight: bold;
    }

    .err,
    .good {
        font-weight: bold;
    }

    .good {
        color: hsl(var(--action));
    }

    /*.muted {*/
    /*    color: hsla(var(--text) / .8)*/
    /*}*/

    /*progress {*/
    /*    accent-color: hsl(var(--accent));*/
    /*}*/

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
        background: rgba(0, 0, 0, 0.85);
        z-index: 20;
    }
</style>
