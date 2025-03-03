<script lang="ts">
    import type {ThemeRequestResponse} from "$api/types/themes.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import BrandingPreview from "$lib5/admin/clients/branding/BrandingPreview.svelte";

    let {
        logoUrl,
        theme,
    }: {
        logoUrl: string,
        theme: ThemeRequestResponse,
    } = $props();

    let tabs = ['Light Theme', 'Dark Theme'];
    let selected = $state(tabs[0]);
</script>

<div>
    <h2>Preview</h2>

    <Tabs {tabs} bind:selected center/>

    <div class="preview">
        {#if selected === tabs[0]}
            <BrandingPreview {logoUrl} borderRadius={theme.border_radius} theme={theme.light} typ="light"/>
        {:else}
            <BrandingPreview {logoUrl} borderRadius={theme.border_radius} theme={theme.dark} typ="dark"/>
        {/if}
    </div>
</div>

<style>
    .preview {
        margin-top: .5rem;
    }
</style>
