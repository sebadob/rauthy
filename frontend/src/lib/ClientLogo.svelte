<script lang="ts">
    import RauthyLogo from "$lib/RauthyLogo.svelte";

    let {
        clientId,
    }: {
        clientId: string,
    } = $props();

    let showDefault = $state(false);
</script>

<div class="logo">
    {#if showDefault}
        <RauthyLogo width="100%"/>
    {:else}
        <img
                src="{`/auth/v1/clients/${clientId}/logo`}"
                alt="Client Logo"
                width="100%"
                height="100%"
                onerror={() => showDefault = true}
        />
    {/if}
</div>

<style>
    .logo {
        margin: 0 .25rem;
        width: 84px;
        height: 84px;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
