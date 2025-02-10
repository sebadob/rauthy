<script lang="ts">
    import type {Snippet} from 'svelte';
    import {page} from "$app/state";

    let {
        href,
        target,
        selectedStep = false,
        hideUnderline = false,
        highlight = false,
        highlightExact = false,
        highlightIncludes,
        highlightWithParams = false,
        children,
    }: {
        href: string,
        target?: string,
        selectedStep?: boolean,
        hideUnderline?: boolean,
        highlight?: boolean,
        highlightExact?: boolean,
        highlightIncludes?: string,
        highlightWithParams?: boolean,
        children: Snippet,
    } = $props();

    let ariaCurrentType: 'page' | 'time' | 'step' | 'location' | 'date' | undefined = $derived.by(() => {
        if (selectedStep) {
            return 'step';
        }

        let route = page.route.id;
        if (!route) {
            return;
        }
        if (highlightWithParams) {
            let hrefPage = `${route}${page.url.search}`;
            if (hrefPage.startsWith(href)) {
                return 'page';
            }
        } else if (highlightExact) {
            if (route === href.split('?')[0]) {
                return 'page';
            }
        } else if (highlightIncludes) {
            if (route.includes(highlightIncludes)) {
                return 'page';
            }
        } else if (route) {
            let link = href.split('?')[0];
            if (link.endsWith(route)) {
                return 'page';
            }
        }
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
        color: hsl(var(--text-high));
        text-decoration: underline;
    }
</style>
