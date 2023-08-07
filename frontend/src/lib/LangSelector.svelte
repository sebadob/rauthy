<script>
    import {onMount} from "svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";

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
        console.log('lang from document: ' + l);
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
            options={['DE', 'EN']}
    />
</div>

<style>
    .absolute {
        position: absolute;
        left: 0;
        bottom: 0;
    }
</style>
