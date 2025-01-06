<script lang="ts">
    import {onMount, type Snippet} from 'svelte'

    import "../css/global.css";

    let {
        children,
    }: {
        children: Snippet,
    } = $props();

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
            Without them, a safe interaction with Rauthy cannot happen.
        </p>
    </div>
{/if}

