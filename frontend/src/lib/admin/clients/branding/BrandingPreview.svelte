<script lang="ts">
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { ThemeCss } from '#api/types/themes.ts';
    import RauthyLogo from '#lib/RauthyLogo.svelte';

    let {
        logoUrl,
        borderRadius,
        theme,
        typ,
    }: {
        logoUrl: string;
        borderRadius: string;
        theme: ThemeCss;
        typ: 'light' | 'dark';
    } = $props();

    const hslVar = (v: number[]) => `${v[0]} ${v[1]}% ${v[2]}%`;

    const t = useI18n();
    let showDefault = $state(false);

    $effect(() => {
        if (logoUrl) showDefault = false;
    });
</script>

<div
    class="branding-preview"
    aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"
    style:--border-radius={borderRadius}
    style:--text={hslVar(theme.text)}
    style:--text-high={hslVar(theme.text_high)}
    style:--bg={hslVar(theme.bg)}
    style:--bg-high={hslVar(theme.bg_high)}
    style:--action={hslVar(theme.action)}
    style:--accent={hslVar(theme.accent)}
    style:--error={hslVar(theme.error)}
    style:--btn-text={theme.btn_text}
    style:--theme-sun={theme.theme_sun}
    style:--theme-moon={theme.theme_moon}
>
    <div class="logo">
        {#if showDefault}
            <RauthyLogo width="3.5rem" show={typ} />
        {:else}
            <img src={logoUrl} alt="Client Logo" onerror={() => (showDefault = true)} />
        {/if}
    </div>

    <div aria-hidden="true" class="sample">
        <h4>{t.authorize.login}</h4>
        <div>
            <span>{t.common.email}</span>
            <div class="input">name@example.com</div>
        </div>
        <div>
            <span>{t.common.password}</span>
            <div class="input">••••••••••</div>
        </div>
        <div class="primary">{t.authorize.login}</div>
        <div class="swatches">
            <span style:background="hsl(var(--accent))"></span>
            <span style:background="hsl(var(--error))"></span>
            <span style:background="var(--theme-sun)"></span>
            <span style:background="var(--theme-moon)"></span>
        </div>
    </div>
</div>

<style>
    .branding-preview {
        padding: 1rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        background: hsl(var(--bg));
        color: hsl(var(--text));
    }

    h4 {
        margin: 0;
        color: hsl(var(--text-high));
    }

    .sample {
        display: grid;
        gap: 0.75rem;
        font-size: 0.8125rem;
        line-height: 1.25rem;
    }

    .sample .input,
    .primary {
        margin: 0.25rem 0 0;
        padding: 0.5rem 0.75rem;
        border-radius: var(--border-radius);
    }

    .sample .input {
        border-color: hsl(var(--text) / 0.25);
        font-size: inherit;
    }

    .primary {
        background: hsl(var(--action));
        color: var(--btn-text);
        font-weight: 600;
        text-align: center;
    }

    .swatches {
        display: flex;
        gap: 6px;
    }

    .swatches span {
        height: 10px;
        width: 10px;
        border-radius: var(--border-radius);
    }

    .logo {
        height: 3.5rem;
        margin-bottom: 0.75rem;
        display: grid;
        place-items: center;
    }

    .logo img {
        height: 100%;
        max-width: 100%;
        object-fit: contain;
    }
</style>
