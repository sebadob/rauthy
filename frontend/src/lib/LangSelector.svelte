<script>
    import {onMount} from "svelte";
    import {getCookie} from "$lib/utils/helpers.js";

    const attrs = ';Path=/;SameSite=Lax;Max-Age=157680000';
    let lang;

    onMount(() => {
        readLang();
    });

    // Tries to an already set language in various places, including legacy Meteo Admin UI locations
    function readLang() {
        // Priority: cookie -> local storage -> Admin UI Cookie -> Admin UI Local Storage
        let cookie = getCookie('locale');
        if (cookie) {
            lang = cookie;
            return;
        }

        // let locale = localStorage.getItem('locale');
        // if (locale) {
        //     document.cookie = 'locale=' + locale + attrs;
        //     lang = locale;
        //     return;
        // }

        lang = navigator.language;
        console.log('navigator.language: ' + lang);
        // If we could not find any value until now, we are assuming 'en' as default
        // document.cookie = 'locale=en' + attrs;
        // lang = 'en';
    }

    function switchLang(l) {
        document.cookie = 'locale=' + l + attrs;
        lang = l;
        window.location.reload();
    }
</script>

<div>
    <div
            role="button"
            tabindex="0"
            class={'de' === lang ? 'lang selected' : 'lang'}
            on:click={() => switchLang('de')}
            on:keypress={() => switchLang('de')}
    >
        DE
    </div>

    <div
            role="button"
            tabindex="0"
            class={'en' === lang ? 'lang selected' : 'lang'}
            on:click={() => switchLang('en')}
            on:keypress={() => switchLang('en')}
    >
        EN
    </div>

    <div
            role="button"
            tabindex="0"
            class={'fr' === lang ? 'lang selected' : 'lang'}
            on:click={() => switchLang('fr')}
            on:keypress={() => switchLang('fr')}
    >
        FR
    </div>
</div>

<style>
    .lang {
        margin: 5px;
        cursor: pointer;
        border-bottom: 1px solid transparent;
    }

    .lang:hover {
        border-bottom: 1px solid var(--col-act2a);
    }

    .selected {
        color: var(--col-act2);
        font-weight: bold;
    }
</style>
