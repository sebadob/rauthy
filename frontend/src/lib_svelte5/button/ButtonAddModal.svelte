<script lang="ts">
    import type {Snippet} from "svelte";
    import Button from "$lib5/button/Button.svelte";
    import Modal from "$lib5/Modal.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

    let {
        level = 2,
        closeModal,
        onClose,
        children,
    }: {
        level?: number,
        closeModal?: undefined | (() => void),
        onClose?: () => void,
        children: Snippet,
    } = $props();

    let ta = useI18nAdmin();
    let showModal = $state(false);
</script>

<Button {level} onclick={() => showModal = true}>
    {ta.common.addNew}
</Button>
<Modal bind:showModal bind:closeModal {onClose}>
    {@render children()}
</Modal>
