<script lang="ts">
    import { page } from '$app/state';
    import type { Snippet } from 'svelte';
    import A from '$lib5/A.svelte';
    import { useTrigger } from '$state/callback.svelte';

    let {
        ref = $bindable(),
        href,
        highlightWithParams = false,
        children,
    }: {
        ref?: undefined | HTMLAnchorElement;
        href: string;
        highlightWithParams?: boolean;
        children: Snippet;
    } = $props();

    let tr = useTrigger();

    let current = $derived(
        highlightWithParams
            ? `${page.route.id}${page.url.search}`.startsWith(href)
            : page.route.id === href.split('?')[0],
    );

    function onclick() {
        // This is a bit of a hacky solution.
        // Our main issue, is that we are trying to solve a chicken-and-egg problem here.
        // -> same as for `<NavLink>`
        requestAnimationFrame(() => {
            setTimeout(() => {
                tr.trigger('navSub');
            }, 100);
        });
    }
</script>

<div class="a" data-current={current}>
    <A bind:ref {href} hideUnderline {highlightWithParams} {onclick}>
        {@render children()}
    </A>
</div>

<style>
    .a {
        width: 100%;
        padding: 0 0.5rem;
    }

    .a[data-current='true'] {
        background: hsla(var(--bg-high) / 0.4);
    }
</style>
