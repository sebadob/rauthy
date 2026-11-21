<script lang="ts">
    import A from '#lib/A.svelte';
    import type { Snippet } from 'svelte';

    let {
        compact,
        params,
        route,
        icon,
        highlightIncludes,
        children,
    }: {
        compact: boolean;
        params: string;
        route: string;
        icon: Snippet<[string]>;
        highlightIncludes?: string;
        children: Snippet;
    } = $props();

    const urlPrefix = '/auth/v1/admin';

    let width = $derived(compact ? '1.5rem' : '1.2rem');
    let href = $derived(`${urlPrefix}${route}${params}`);
</script>

<A {href} hideUnderline {highlightIncludes}>
    {#if compact}
        <div class="compact">
            <div class="iconCompact">
                {@render icon(width)}
            </div>
            <span>
                {@render children()}
            </span>
        </div>
    {:else}
        <div class="wide">
            <div class="iconWide" style:width>
                {@render icon(width)}
            </div>
            <div>
                {@render children()}
            </div>
        </div>
    {/if}
</A>

<style>
    .compact {
        display: flex;
        flex-direction: column;
        align-items: center;
        font-size: 0.7rem;
    }

    .compact > span {
        max-width: 100%;
        white-space: nowrap;
        overflow-x: hidden;
        text-overflow: ellipsis;
    }

    .compact > span {
        color: hsla(var(--text) / 0.66);
    }

    .iconCompact {
        margin-bottom: -0.2rem;
    }

    .iconWide {
        transform: translateY(0.15rem);
    }

    .wide {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        overflow: clip;
    }
</style>
