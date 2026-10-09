<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import type { ThemeRequestResponse } from '$api/types/themes.ts';
    import Tabs from '$lib5/tabs/Tabs.svelte';
    import BrandingPreview from '$lib5/admin/clients/branding/BrandingPreview.svelte';

    let ta = useI18nAdmin();

    let {
        logoUrl,
        theme,
    }: {
        logoUrl: string;
        theme: ThemeRequestResponse;
    } = $props();

    let tabs = [ta.clients.branding.lightTheme, ta.clients.branding.darkTheme];
    let selected = $state(tabs[0]);
</script>

<div>
    <h2>{ta.common.preview}</h2>

    <div class="tabs">
        <Tabs {tabs} bind:selected center />
    </div>

    <div class="preview">
        {#if selected === tabs[0]}
            <BrandingPreview
                {logoUrl}
                borderRadius={theme.border_radius}
                theme={theme.light}
                typ="light"
            />
        {:else}
            <BrandingPreview
                {logoUrl}
                borderRadius={theme.border_radius}
                theme={theme.dark}
                typ="dark"
            />
        {/if}
    </div>
</div>

<style>
    .preview {
        margin-top: 0.5rem;
    }

    .tabs {
        width: 17rem;
    }
</style>
