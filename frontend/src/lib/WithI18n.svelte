<script>
    import {onMount} from "svelte";
    import {base} from "$app/paths";


    export let content;
    export let t;
    // export let tDyn;
    export let isReady = false;
    // export let withLangSelector = false;

    // static translation
    onMount(async () => {
        if ('production' === import.meta.env.MODE) {
            t = JSON.parse(document.querySelector("#i18n").innerHTML);
        } else {
            const res = await fetchI18nStatic(content);
            t = await res.json();
        }
        isReady = true;
    });

    // // dynamic translations
    // onMount(async () => {
    //     const res = await fetchI18n(content);
    //     tDyn = await res.json();
    // });

    // async function fetchI18n(content) {
    //     let data = {content};
    //     return await fetch(`${base}/i18n`, {
    //         method: 'POST',
    //         headers: {
    //             'Content-Type': 'application/json',
    //             'Accept': 'application/json',
    //         },
    //         body: JSON.stringify(data),
    //     });
    // }

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
    <!--{#if withLangSelector}-->
    <!--    <LangSelector/>-->
    <!--{/if}-->

    <slot></slot>
{/if}
