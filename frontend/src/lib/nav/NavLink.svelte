<script lang="ts">
    import A from "$lib5/A.svelte";
    import type {Snippet} from "svelte";

    let {
        compact,
        params,
        route,
        icon,
        highlightIncludes,
        children,
    }: {
        compact: boolean,
        params: string,
        route: string,
        icon: Snippet<[string]>,
        highlightIncludes?: string,
        children: Snippet,
    } = $props();

    const urlPrefix = '/auth/v1/admin';

    let width = $derived(compact ? "1.5rem" : "1.2rem");
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
        font-size: .7rem;
        word-break: break-word;
    }

    .compact > span {
        color: hsla(var(--text) / .66);
    }

    .iconCompact {
        margin-bottom: -.2rem;
    }

    .iconWide {
        transform: translateY(.15rem);
    }

    .wide {
        display: flex;
        align-items: center;
        gap: .5rem;
        overflow: clip;
    }
</style>
