<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {page} from "$app/stores";
    import type {Snippet} from "svelte";
    import A from "$lib5/A.svelte";

    let {
        href,
        highlightExact = true,
        highlightWithParams = false,
        children,
    }: {
        href: string,
        highlightExact?: boolean,
        highlightWithParams?: boolean,
        children: Snippet,
    } = $props();

    let current = $derived(highlightWithParams
        ? `${$page.route.id}${$page.url.search}`.startsWith(href)
        : $page.route.id === href.split('?')[0]
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
