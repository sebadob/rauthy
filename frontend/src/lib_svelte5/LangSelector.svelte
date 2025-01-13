<script lang="ts">
    import {untrack} from "svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import {LANGUAGES} from "../utils/constants.js";
    import {postUpdateUserLanguage} from "../utils/dataFetching.js";
    import {useLang} from "$state/language.svelte.js";
    import {getCookie} from "$lib/utils/helpers";

    let {
        absolute = false,
        absoluteRight = false,
        updateBackend = false,
    }: {
        absolute: string,
        absoluteRight: string,
        updateBackend: string,
    } = $props();

    const attrs = ';Path=/;SameSite=Lax;Max-Age=315360000';
    let lang = useLang();
    let langSelected = $state(untrack(() => lang));

    // Makes lang overrides work during local dev
    // Will do nothing in prod because of proper SSR.
    $effect(() => {
        let cookie = getCookie('locale');
        let l = untrack(() => lang);
        if (cookie !== l && LANGUAGES.includes(cookie)) {
            lang = cookie;
            langSelected = cookie;
        }
    });

    $effect(() => {
        if (langSelected && langSelected !== lang) {
            switchLang(langSelected);
        }
    });

    async function switchLang(l: string) {
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
