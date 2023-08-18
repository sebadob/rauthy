<script>
    import {onMount} from "svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import {LANGUAGES} from "../utils/constants.js";
    import {postUpdateUserLanguage} from "../utils/dataFetching.js";

    export let absolute = false;
    // if set to true, a request to the backend will be done for the currently logged in user
    export let updateBackend = false;

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

    async function switchLang(l) {
        document.cookie = 'locale=' + l.toLowerCase() + attrs;

        if (updateBackend) {
            let res = await postUpdateUserLanguage();
            if (!res.ok) {
                let body = await res.json();
                console.error(body);
                return;
            }

        }

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
