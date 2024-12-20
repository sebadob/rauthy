<script>
    import { run } from 'svelte/legacy';

    import {onMount} from "svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import {LANGUAGES} from "../utils/constants.js";
    import {postUpdateUserLanguage} from "../utils/dataFetching.js";

    
    /**
     * @typedef {Object} Props
     * @property {boolean} [absolute]
     * @property {boolean} [absoluteRight]
     * @property {boolean} [updateBackend] - if set to true, a request to the backend will be done for the currently logged in user
     */

    /** @type {Props} */
    let { absolute = false, absoluteRight = false, updateBackend = false } = $props();

    const attrs = ';Path=/;SameSite=Lax;Max-Age=157680000';
    let lang = $state();
    let langSelected = $state();

    onMount(async () => {
        readLang();
    });


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
    run(() => {
        if (langSelected && langSelected !== lang) {
            switchLang(langSelected);
        }
    });
</script>

<div class:absolute class:absoluteLeft={!absoluteRight} class:absoluteRight>
    <OptionSelect
            bind:value={langSelected}
            options={LANGUAGES}
    />
</div>

<style>
    .absolute {
        position: absolute;
        top: calc(100dvh - 1.75rem);
    }

    .absoluteLeft {
        left: .25rem;
    }

    .absoluteRight {
        right: .25rem;
    }
</style>
