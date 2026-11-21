<script lang="ts">
    import type { ThemeCss } from '#api/types/themes.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import HSLInputRange from '#lib/admin/clients/branding/HSLInputRange.svelte';
    import HSLInputText from '#lib/admin/clients/branding/HSLInputText.svelte';
    import { genKey } from '#utils/helpers.ts';

    let {
        values = $bindable(),
    }: {
        values: ThemeCss;
    } = $props();

    let ta = useI18nAdmin();
    const palette = [
        { key: 'text', label: '--text' },
        { key: 'text_high', label: '--text-high' },
        { key: 'bg', label: '--bg' },
        { key: 'bg_high', label: '--bg-high' },
        { key: 'action', label: '--action' },
        { key: 'accent', label: '--accent' },
        { key: 'error', label: '--error' },
    ] as const;
    const group = genKey();
</script>

<div>
    <div class="hsl">
        {#each palette as { key, label } (key)}
            <HSLInputRange
                {label}
                {group}
                bind:h={values[key][0]}
                bind:s={values[key][1]}
                bind:l={values[key][2]}
            />
        {/each}
    </div>

    <details>
        <summary>{ta.clients.branding.advanced}</summary>
        <p>{ta.clients.branding.descHsl}</p>
        <p>{@html ta.clients.branding.descVariables}</p>
        <p>{@html ta.clients.branding.descFullCss}</p>
        <HSLInputText label="--btn-text" bind:value={values.btn_text} />
        <HSLInputText label="--theme-sun" bind:value={values.theme_sun} />
        <HSLInputText label="--theme-moon" bind:value={values.theme_moon} />
    </details>
</div>

<style>
    .hsl {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 12rem), 1fr));
        align-items: start;
        gap: 0.5rem;
    }
    details {
        margin-top: var(--ui-space);
    }
    summary {
        cursor: pointer;
        color: var(--ui-primary);
    }
</style>
