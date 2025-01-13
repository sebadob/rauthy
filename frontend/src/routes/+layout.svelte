<script lang="ts">
    import {onMount, type Snippet} from 'svelte'

    import "../css/global.css";
    import {useIsDev} from "../global_state/is_dev.svelte.ts";

    let {
        children,
    }: {
        children: Snippet,
    } = $props();

    let isDev = useIsDev();
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
