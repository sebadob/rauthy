<script>
    import {onMount} from "svelte";
    /**
     * @typedef {Object} Props
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props} */
    let { children } = $props();

    let isChecked = $state(false);
    let isOk = $state(false);

    onMount(() => {
        isOk = navigator.cookieEnabled;
        isChecked = true;
    });

</script>

<noscript>
    <h2>You need to enable JavaScript</h2>
</noscript>

{#if isChecked}
    {#if isOk}
        {@render children?.()}
    {:else}
        <div>
            <h2>Cookies disabled</h2>
            <p>
                You need to enable Cookies.<br>
                Without them, a safe login cannot be executed.
            </p>
        </div>
    {/if}
{/if}
