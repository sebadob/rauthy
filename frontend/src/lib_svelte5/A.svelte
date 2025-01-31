<script lang="ts">
    import type {Snippet} from 'svelte';
    import {page} from "$app/state";

    let {
        href,
        target,
        selectedStep = false,
        hideUnderline = false,
        highlight = false,
        highlightExact = true,
        highlightWithParams = false,
        children,
    }: {
        href: string,
        target?: string,
        selectedStep?: boolean,
        hideUnderline?: boolean,
        highlight?: boolean,
        highlightExact?: boolean,
        highlightWithParams?: boolean,
        children: Snippet,
    } = $props();

    let ariaCurrentType: 'page' | 'time' | 'step' | 'location' | 'date' | undefined = $derived.by(() => {
        if (selectedStep) {
            return 'step';
        }

        if (highlightWithParams) {
            let hrefPage = `${page.route.id}${page.url.search}`;
            if (hrefPage.startsWith(href)) {
                return 'page';
            }
        } else if (highlightExact) {
            if (page.route.id === href.split('?')[0]) {
                return 'page';
            }
        } else if (page.route.id) {
            if (page.route.id.startsWith(href.split('?')[0])) {
                return 'page';
            }
        }

        return undefined;
    });

</script>

<a
        class="font-label"
        class:hideUnderline
        {href}
        {target}
        aria-current={ariaCurrentType}
        data-highlight={highlight}
>
    {@render children()}
</a>

<style>
    /*a, a:link, a:visited {*/
    /*    color: hsl(var(--text));*/
    /*    !*transition: all 150ms ease-in-out;*!*/
    /*}*/

    a[data-highlight="true"], a[data-highlight="true"]:link, a[data-highlight="true"]:visited {
        color: hsl(var(--action));
    }

    .hideUnderline {
        text-decoration: none;
    }

    /*a:hover, a:active {*/
    /*    color: hsl(var(--action));*/
    /*    text-decoration: underline;*/
    /*}*/

    a[aria-current="page"],
    a[aria-current="step"] {
        text-decoration: underline;
    }
</style>
