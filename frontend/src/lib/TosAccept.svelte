<script lang="ts">
    import type {
        ToSAwaitLoginResponse,
        ToSLatestResponse,
        ToSUserAcceptRequest,
    } from '$api/types/tos';
    import Modal from '$lib/Modal.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { fetchPost, type IResponse } from '$api/fetch';
    import { onMount } from 'svelte';
    import { formatDateFromTs } from '$utils/helpers';
    import type { WebauthnLoginResponse } from '$api/types/authorize';
    import MarkdownRenderer from '$lib/text_edit/md/MarkdownRenderer.svelte';

    let {
        tos,
        tosAcceptCode = '',
        forceAccept,
        onToSAccept,
        onToSCancel,
        skipRequest,
    }: {
        tos: ToSLatestResponse;
        tosAcceptCode?: string;
        forceAccept?: boolean;
        onToSAccept: (
            res?: IResponse<undefined | ToSAwaitLoginResponse | WebauthnLoginResponse>,
        ) => Promise<void>;
        onToSCancel?: () => void;
        skipRequest?: boolean;
    } = $props();

    let t = useI18n();

    let refToS: undefined | HTMLParagraphElement = $state();
    let showModal = $state(true);
    let closeModal: undefined | (() => void) = $state();

    let now = $state(new Date().getTime() / 1000);
    let i = setInterval(() => {
        now = new Date().getTime() / 1000;
    }, 1000);
    let acceptOptional = $derived(
        !(forceAccept || tos.opt_until === undefined || tos.opt_until <= now - 3),
    );

    let tosRead = $state(false);

    onMount(() => {
        return clearInterval(i);
    });

    $effect(() => {
        if (refToS) {
            setTimeout(() => {
                onScrollEndToS();
            }, 1000);
        }
    });

    function onScrollEndToS() {
        if (!refToS) {
            return false;
        }

        // allow 50px diff for better UX
        if (!tosRead && refToS.scrollHeight <= refToS.scrollTop + refToS.offsetHeight + 50) {
            tosRead = true;
        }
    }

    async function onAccept() {
        if (!tos) {
            return;
        }

        if (!skipRequest) {
            let payload: ToSUserAcceptRequest = {
                accept_code: tosAcceptCode,
                tos_ts: tos.ts,
            };
            let res = await fetchPost<undefined>('/auth/v1/tos/accept', payload);
            // TODO handle accept code expiration
            await onToSAccept(res);
        } else {
            await onToSAccept();
        }

        closeModal?.();
    }

    function onCancel() {
        closeModal?.();
        onToSCancel?.();
    }

    async function onDeny() {
        if (!tos) {
            return;
        }

        let payload: ToSUserAcceptRequest = {
            accept_code: tosAcceptCode,
            tos_ts: tos.ts,
        };
        let res = await fetchPost<undefined>('/auth/v1/tos/deny', payload);
        // TODO handle accept code expiration

        if (res.error) {
            // this should never happen because of our pre-checks
            tos.opt_until = undefined;
        } else {
            closeModal?.();
            await onToSAccept(res);
        }
    }
</script>

{#if tos}
    <Modal bind:showModal bind:closeModal strict>
        <h1>{t.tos.tos}</h1>
        <div bind:this={refToS} class="tosContent html-render" onscrollend={onScrollEndToS}>
            {#if tos.is_html}
                <MarkdownRenderer markdown={tos.content} />
            {:else}
                {tos.content}
            {/if}
        </div>

        <div class="flex gap-05">
            <Button ariaLabel={t.common.accept} onclick={onAccept} isDisabled={!tosRead}>
                {t.common.accept}
            </Button>

            {#if acceptOptional}
                <Button level={-2} ariaLabel={t.tos.deny} onclick={onDeny}>
                    {t.tos.deny}
                </Button>

                {#if tos.opt_until}
                    <div class="acceptOpt">
                        {t.tos.acceptOptUntil}
                        {formatDateFromTs(tos.opt_until)}
                    </div>
                {/if}
            {:else}
                <Button level={-2} ariaLabel={t.common.cancel} onclick={onCancel}>
                    {t.common.cancel}
                </Button>
            {/if}
        </div>
    </Modal>
{/if}

<style>
    .acceptOpt {
        font-size: 0.9rem;
        color: hsl(var(--text) / 0.8);
    }

    .tosContent {
        width: min(90dvw, 467pt);
        max-height: min(75dvh, 40rem);
        margin-bottom: 1rem;
        padding-right: 1rem;
        overflow-y: auto;
    }
</style>
