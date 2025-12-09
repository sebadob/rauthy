<script lang="ts">
    import { useI18n } from '$state/i18n.svelte';
    import { untrack } from 'svelte';
    import { getCookie } from '$utils/helpers';
    import Options from '$lib5/Options.svelte';
    import { fetchPost } from '$api/fetch';
    import { page } from '$app/state';
    import type { Language } from '$api/types/i18n.ts';
    import { useI18nConfig } from '$state/i18n_config.svelte';

    let {
        absolute,
        openTop = true,
        updateBackend,
        borderless = false,
    }: {
        absolute?: boolean;
        openTop?: boolean;
        updateBackend?: boolean;
        borderless?: boolean,
    } = $props();

    const cookieAttrs = ';Path=/;SameSite=Lax;Max-Age=157680000';

    let t = useI18n();
    let initial = untrack(() => t.lang);
    let selected = $state(initial);

    let languages: undefined | Language[] = $derived(
        page.route.id?.includes('/admin') ? useI18nConfig().admin() : useI18nConfig().common(),
    );

    let offsetTop = $derived(openTop ? `-${(languages?.length || 0) * 2 + 2}rem` : undefined);
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

{#if languages}
    <div class:absolute>
        <Options
            ariaLabel={t.common.selectI18n}
            options={languages}
            borderless={borderless}
            bind:value={selected}
            {offsetTop}
            {offsetLeft}
        />
    </div>
{/if}

<style>
    .absolute {
        height: 1.5rem;
        position: absolute;
        bottom: 0.35rem;
        left: 2rem;
        overflow: clip;
    }
</style>
