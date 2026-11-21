<script lang="ts">
    import Modal from '#lib/Modal.svelte';
    import Button from '#lib/button/Button.svelte';
    import { fetchPost, fetchPut } from '#api/fetch.ts';
    import type { PamHostSecretResponse } from '#api/types/pam.ts';
    import HiddenValueArea from '#lib/HiddenValueArea.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        hostId,
    }: {
        hostId: string;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();
    let showModal = $state(false);

    let err = $state('');
    let secret = $state('');
    let url = $derived(`/auth/v1/pam/hosts/${hostId}/secret`);

    async function fetchSecret() {
        let res = await fetchPost<PamHostSecretResponse>(url);
        if (res.body) {
            secret = res.body.secret;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onclick() {
        err = '';
        secret = '';
        void fetchSecret();
        showModal = true;
    }

    async function rotate() {
        let res = await fetchPut<PamHostSecretResponse>(url);
        if (res.body) {
            secret = res.body.secret;
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<Button level={2} {onclick}>
    {ta.pam.secretShow}
</Button>
<Modal bind:showModal bind:closeModal ariaLabel="Secret">
    <div class="host-secret">
        <h2>Secret</h2>
        {#key secret}
            <HiddenValueArea ariaLabel="Secret" rows={3} value={secret} width="100%" />
        {/key}

        {#if err}
            <p class="err" role="alert">{err}</p>
        {/if}
        <div class="actions">
            <Button level={2} onclick={() => closeModal?.()}>{t.common.close}</Button>
            <Button onclick={rotate}>{ta.pam.secretRotate}</Button>
        </div>
    </div>
</Modal>

<style>
    .host-secret {
        width: min(39rem, calc(100dvw - 6rem - 2px));
        display: grid;
        gap: var(--ui-space);
    }

    h2,
    .err {
        margin: 0;
    }

    .actions {
        display: flex;
        flex-wrap: wrap;
        justify-content: end;
        align-items: center;
        gap: 0.5rem;
    }
</style>
