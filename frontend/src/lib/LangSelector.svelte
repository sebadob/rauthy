<script>
    import {onMount} from "svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import {LANGUAGES} from "../utils/constants.js";

    export let absolute = false;

    const attrs = ';Path=/;SameSite=Lax;Max-Age=157680000';
    let lang;
    let langSelected;

    onMount(async () => {
        readLang();
    });

    $: if (langSelected && langSelected !== lang) {
        switchLang(langSelected);
    }

    function readLang() {
        let l = document.documentElement.lang.toUpperCase();
        lang = l;
        langSelected = l;
    }

    function switchLang(l) {
        let ll = l.toLowerCase();
        console.log('switching language to: ' + ll);
        document.cookie = 'locale=' + ll + attrs;
        window.location.reload();
    }
</script>

<div class:absolute>
    <OptionSelect
            bind:value={langSelected}
            options={LANGUAGES}
    />
</div>

<style>
    .absolute {
        position: absolute;
        top: calc(100dvh - 1.5rem);
        left: 0;
    }
</style>
