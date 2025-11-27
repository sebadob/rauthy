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
        updated ? `/auth/v1/clients/${clientId}/logo?updated=${updated}` : `/auth/v1/clients/${clientId}/logo`,
    );
</script>

<div class="logo">
    {#if showDefault || updated === undefined}
        <RauthyLogo width="100%" />
    {:else}
        <img
            {src}
            alt="Client Logo"
            width="100%"
            height="100%"
            onerror={() => (showDefault = true)}
        />
    {/if}
</div>

<style>
    .logo {
        margin: 0 0.25rem;
        width: 84px;
        height: 84px;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
