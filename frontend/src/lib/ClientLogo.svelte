<script lang="ts">
    import RauthyLogo from '$lib/RauthyLogo.svelte';

    let {
        clientId,
        updated,
    }: {
        clientId: string;
        updated: undefined | number;
    } = $props();

    let showDefault = $state(false);
    let src = $derived(
        updated
            ? `/auth/v1/clients/${clientId}/logo?updated=${updated}`
            : `/auth/v1/clients/${clientId}/logo`,
    );
</script>

<div class="logo" class:default={showDefault}>
    {#if showDefault}
        <RauthyLogo width="100%" />
    {:else}
        <img {src} alt="Client Logo" onerror={() => (showDefault = true)} />
    {/if}
</div>

<style>
    .logo {
        margin: 0 0.25rem;
        height: 84px;
        max-width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .logo.default {
        width: 84px;
    }

    img {
        height: 100%;
        max-width: 100%;
        object-fit: contain;
    }
</style>
