<script lang="ts">
    import {page} from "$app/state";
    import type {Snippet} from "svelte";
    import A from "$lib5/A.svelte";

    let {
        href,
        highlightWithParams = false,
        children,
    }: {
        href: string,
        highlightWithParams?: boolean,
        children: Snippet,
    } = $props();

    let current = $derived(highlightWithParams
        ? `${page.route.id}${page.url.search}`.startsWith(href)
        : page.route.id === href.split('?')[0]
    );

</script>

<div class="a" data-current={current}>
    <A {href} hideUnderline {highlightWithParams}>
        {@render children()}
    </A>
</div>

<style>
    .a {
        width: 100%;
        padding: 0 .5rem;
    }

    .a[data-current="true"] {
        background: hsla(var(--bg-high) / .4);
    }
</style>
