<script lang="ts">
    import { type Snippet, untrack } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        alignRight,
        level = 2,
        closeModal = $bindable(),
        onClose,
        ref = $bindable(),
        children,
    }: {
        alignRight?: boolean;
        level?: number;
        closeModal?: undefined | (() => void);
        onClose?: () => void;
        ref?: undefined | HTMLButtonElement;
        children: Snippet;
    } = $props();

    let ta = useI18nAdmin();

    const levelModal = untrack(() => (level === 1 ? 2 : 3));
    let showModal = $state(false);
</script>

<div class="admin-add-button" class:alignRight>
    <Button bind:ref level={showModal ? levelModal : level} onclick={() => (showModal = true)}>
        <span class="admin-add-symbol" aria-hidden="true">+</span>{ta.common.addNew}
    </Button>
    <Modal bind:showModal bind:closeModal {onClose}>
        {@render children()}
    </Modal>
</div>

<style>
    div {
        margin: 0 0.25rem;
    }

    .alignRight {
        text-align: right;
    }

    :global(.admin-shell) .admin-add-button > :global(.admin-control) {
        color: var(--ui-on-primary);
        background: var(--ui-primary);
        border: 0;
    }
    :global(.admin-shell) .admin-add-symbol {
        display: grid;
        place-items: center;
        width: 18px;
        height: 18px;
        font: 400 22px/18px sans-serif;
        margin: 0;
    }
    :global(.admin-shell) .admin-add-button > :global(.admin-control[data-variant]) {
        background: var(--ui-primary);
        color: var(--ui-on-primary);
        border-color: transparent;
    }

    :global(.admin-shell)
        .admin-add-button
        > :global(.admin-control[data-variant]:hover:not(:disabled)) {
        background: color-mix(in srgb, var(--ui-primary) 93%, white);
    }
</style>
