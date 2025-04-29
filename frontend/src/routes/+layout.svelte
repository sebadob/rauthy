<script lang="ts">
    import {onMount, type Snippet} from 'svelte'
    import {initLang} from "$state/language.svelte";
    import {initI18n} from "$state/i18n.svelte";

    import "../css/global.css";

    initLang();
    initI18n();

    let {
        children,
    }: {
        children: Snippet,
    } = $props();

    // We expect true to not break SSR.
    // If cookies should be disabled, the warning will show up fast enough.
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
