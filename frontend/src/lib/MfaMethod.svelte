<script lang="ts">
    import type { Snippet } from 'svelte';
    import Section from '#lib/Section.svelte';
    import Loading from '#lib/Loading.svelte';

    let {
        title,
        empty,
        loading,
        error,
        actions,
        children,
    }: {
        title: string;
        empty?: string;
        loading: boolean;
        error?: string;
        actions?: Snippet;
        children?: Snippet;
    } = $props();
</script>

<Section {title} {actions}>
    <div class="method" aria-busy={loading}>
        {#if loading}
            <Loading />
        {:else if error}
            <p class="err" role="alert">{error}</p>
        {:else if empty}
            <p class="muted">{empty}</p>
        {/if}
        {@render children?.()}
    </div>
</Section>

<style>
    .method {
        display: grid;
        gap: 0.75rem;
        min-width: 0;
        font-size: var(--ui-control-font-size);
        line-height: 1.5;
    }

    .muted {
        color: var(--ui-muted);
    }
</style>
