<script lang="ts">
    import { onMount } from 'svelte';
    import RauthyLogo from '#lib/RauthyLogo.svelte';

    let {
        clientId,
        updated,
    }: {
        clientId: string | undefined;
        updated: undefined | number;
    } = $props();

    let image = $state<HTMLImageElement>();
    let failedSrc = $state<string>();
    let src = $derived.by(() => {
        if (!clientId) return undefined;

        const path = `/auth/v1/clients/${clientId}/logo`;
        return updated === undefined ? path : `${path}?updated=${updated}`;
    });
    let showDefault = $derived(!src || failedSrc === src);

    onMount(() => {
        // The prerendered image can fail before hydration attaches the error handler.
        if (image?.complete && image.naturalWidth === 0) {
            failedSrc = image.getAttribute('src') ?? undefined;
        }
    });
</script>

<div class="logo" class:default={showDefault}>
    {#if showDefault}
        <RauthyLogo width="100%" />
    {:else}
        <img bind:this={image} {src} alt="Client Logo" onerror={() => (failedSrc = src)} />
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
