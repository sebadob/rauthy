<script lang="ts">
    import {onMount, type Snippet} from 'svelte'
    import {initIsDev} from "$state/is_dev.svelte.ts";
    import {initLang} from "$state/language.svelte.ts";
    import {initI18n} from "$state/i18n.svelte.ts";

    import "../css/global.css";

    let {
        children,
    }: {
        children: Snippet,
    } = $props();

    initIsDev();
    initLang();
    initI18n();

    let cookiesEnabled = $state(true);

    onMount(() => {
        cookiesEnabled = navigator.cookieEnabled;
    });
</script>

<noscript>
    <h2>You need to enable JavaScript</h2>
</noscript>

{#if cookiesEnabled}
    {@render children()}
{:else}
    <div>
        <h1>Cookies disabled</h1>
        <p>
            You need to enable Cookies.<br>
            Without them, a safe interaction with Rauthy is not possible.
        </p>
    </div>
{/if}
