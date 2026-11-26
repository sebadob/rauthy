<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { fetchDelete } from '#api/fetch.ts';
    import Modal from '#lib/Modal.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import IconTrash from '#icons/IconTrash.svelte';
    import type { Snippet } from 'svelte';

    let {
        url,
        name,
        message,
        disabledReason,
        iconOnly,
        label,
        ariaLabel,
        confirmDisabled,
        details,
        onOpen,
        onDelete,
    }: {
        url: string;
        name?: string;
        message?: string;
        disabledReason?: string;
        iconOnly?: boolean;
        label?: string;
        ariaLabel?: string;
        confirmDisabled?: boolean;
        details?: Snippet;
        onOpen?: () => void;
        onDelete: () => void;
    } = $props();

    let t = useI18n();
    const action = $derived(label ?? t.common.delete);

    let err = $state('');
    let showModal = $state(false);
    let closeModal: (() => void) | undefined = $state();
    let pending = $state(false);

    async function onSubmit() {
        err = '';
        pending = true;

        let res = await fetchDelete(url);
        pending = false;
        if (res.error) {
            err = res.error.message;
        } else {
            closeModal?.();
            onDelete();
        }
    }
</script>

{#if disabledReason}
    <Tooltip text={disabledReason}>
        <Button level={-3} isDisabled ariaLabel={action}>
            <IconTrash width="1rem" />
            {#if !iconOnly}{action}{/if}
        </Button>
    </Tooltip>
{:else}
    <Button
        level={-3}
        ariaLabel={ariaLabel || action}
        invisible={iconOnly}
        onclick={() => {
            err = '';
            showModal = true;
            onOpen?.();
        }}
    >
        <IconTrash width="1rem" />
        {#if !iconOnly}{action}{/if}
    </Button>
    <Modal bind:showModal bind:closeModal ariaLabel={action} strict>
        <div class="confirmation">
            <h3>{action}</h3>
            {#if message}<p>{message}</p>{/if}
            {#if name}<p><strong>{name}</strong></p>{/if}
            {#if err}<p class="err" role="alert">{err}</p>{/if}
            {@render details?.()}
            <div class="actions">
                <Button level={2} onclick={() => closeModal?.()} isDisabled={pending}>
                    {t.common.cancel}
                </Button>
                <Button
                    level={-1}
                    onclick={onSubmit}
                    isLoading={pending}
                    isDisabled={confirmDisabled}
                >
                    {action}
                </Button>
            </div>
        </div>
    </Modal>
{/if}

<style>
    .confirmation {
        width: min(24rem, calc(100dvw - 4rem));
        overflow-wrap: anywhere;
    }
    .confirmation h3 {
        margin-top: 0;
    }
    .actions {
        display: flex;
        justify-content: end;
        gap: 0.5rem;
        margin-top: 1.25rem;
    }
</style>
