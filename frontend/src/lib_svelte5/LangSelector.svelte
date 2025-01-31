<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {untrack} from "svelte";
    import {getCookie} from "$utils/helpers.ts";
    import {LANGUAGES} from "$utils/constants";
    import Options from "$lib5/Options.svelte";
    import {fetchPost} from "$api/fetch.ts";

    let {
        absolute,
        openTop = true,
        updateBackend,
    }: {
        absolute?: boolean,
        openTop?: boolean,
        updateBackend?: boolean,
    } = $props();

    const cookieAttrs = ';Path=/;SameSite=Lax;Max-Age=157680000';

    let t = useI18n();
    let initial = untrack(() => t.lang);
    let selected = $state(initial);

    let offsetTop = $derived(openTop ? `-${LANGUAGES.length * 2 + 2}rem` : undefined);
    let offsetLeft = $derived(openTop ? '.2rem' : undefined);

    $effect(() => {
        let cookie = getCookie('locale');
        let current = untrack(() => selected);
        if (cookie && cookie !== current) {
            document.cookie = 'locale=' + current + cookieAttrs;
        }
    });

    $effect(() => {
        if (selected !== initial) {
            switchLang();
        }
    });

    async function switchLang() {
        // locale is technically mandatory by EU cookie laws
        // -> we can just save without cookie consent
        document.cookie = 'locale=' + selected.toLowerCase() + cookieAttrs;

        if (updateBackend) {
            let res = await fetchPost('/auth/v1/update_language');
            if (res.error) {
                console.error(res.error);
                return;
            }
        }

        window.location.reload();
    }
</script>

<div class:absolute>
    <Options
            ariaLabel={t.common.selectI18n}
            options={LANGUAGES}
            borderless
            bind:value={selected}
            {offsetTop}
            {offsetLeft}
    />
</div>

<style>
    .absolute {
        height: 1.5rem;
        position: absolute;
        bottom: .35rem;
        left: 2rem;
        overflow: clip;
    }
</style>