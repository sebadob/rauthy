<script>
    import {onMount} from "svelte";
    import {base} from "$app/paths";
    import LangSelector from '$common/LangSelector.svelte';

    // It may make sense to make a real copy of this component for your project.
    // Just copy & paste it into your current project, so you can adjust the URLs for the translations from the backend.
    // These of course highly depend on your application any may need adjustment. Just copying this component is way
    // easier and more efficient than making this more generic with things like environment variables.
    // It is just this very file you need to copy into your own `$lib` folder.

    export let content;
    export let t;
    export let tDyn;
    export let isReady = false;
    export let withLangSelector = false;

    // static translation
    onMount(async () => {
        if ('production' === import.meta.env.MODE) {
            t = JSON.parse(document.getElementById('i18n').innerText);
        } else {
            const res = await fetchI18nStatic(content);
            t = await res.json();
        }
        isReady = true;
    });

    // dynamic translations
    onMount(async () => {
        const res = await fetchI18n(content);
        tDyn = await res.json();
    });

    async function fetchI18n(content) {
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

    async function fetchI18nStatic(content) {
        let data = {content};
        return await fetch(`${base}/i18n/static`, {
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
    {#if withLangSelector}
        <LangSelector/>
    {/if}

    <slot></slot>
{/if}
