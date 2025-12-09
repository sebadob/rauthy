<script lang="ts">
    import { type Snippet, untrack } from 'svelte';
    import Button from '$lib5/button/Button.svelte';
    import Modal from '$lib5/Modal.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';

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

<div class:alignRight>
    <Button bind:ref level={showModal ? levelModal : level} onclick={() => (showModal = true)}>
        {ta.common.addNew}
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
</style>
