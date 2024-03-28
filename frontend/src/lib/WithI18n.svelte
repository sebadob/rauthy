<script>
    import {onMount} from "svelte";
    import {base} from "$app/paths";


    export let content;
    export let t;
    export let isReady = false;

    // static translation
    onMount(async () => {
        if ('production' === import.meta.env.MODE) {
            t = JSON.parse(document.getElementsByTagName('template').namedItem('i18n').innerHTML);
            // t = JSON.parse(document.querySelector("#i18n").innerHTML);
        } else {
            const res = await fetchI18nStatic(content);
            t = await res.json();
        }
        isReady = true;
    });

    async function fetchI18nStatic(content) {
        let data = {content};
        return await fetch(`${base}/i18n`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            },
            body: JSON.stringify(data),
        });
    }

</script>

{#if !isReady}
    <slot name="loading"></slot>
{:else}
    <slot></slot>
{/if}
