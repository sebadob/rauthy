<script lang="ts">
    import { onMount, type Snippet } from 'svelte';
    import { initLang } from '$state/language.svelte';
    import { initI18n } from '$state/i18n.svelte';

    import '../css/global.css';

    initLang();
    initI18n();

    let {
        children,
    }: {
        children: Snippet;
    } = $props();

    let isSecureContext = $state(true);

    // We expect true to not break SSR.
    // If cookies should be disabled, the warning will show up fast enough.
    let cookiesEnabled = $state(true);

    onMount(() => {
        cookiesEnabled = navigator.cookieEnabled;
        isSecureContext = window.crypto?.subtle !== undefined;
    });
</script>

<noscript>
    <h2>You need to enable JavaScript</h2>
</noscript>

{#if !isSecureContext}
    <div class="err">
        <h1>Not a secure browser context</h1>
        <p>
            You are either not in a secure browser context, or <code>window.crypto.subtle.</code>
            is not available.
        </p>
    </div>
{:else if cookiesEnabled}
    {@render children()}
{:else}
    <div>
        <h1>Cookies disabled</h1>
        <p>
            You need to enable Cookies.<br />
            Without them, a safe interaction with Rauthy is not possible.
        </p>
    </div>
{/if}
