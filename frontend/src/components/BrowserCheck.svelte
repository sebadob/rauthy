<script>
    import {onMount} from "svelte";

    let isChecked = false;
    let isOk = false;

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
        <slot></slot>
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
