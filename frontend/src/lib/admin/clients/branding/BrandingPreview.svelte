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
    data-typ={typ}
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
    <div aria-hidden="true" class="card">
        <div class="logo">
            {#if showDefault}
                <RauthyLogo width="2.75rem" show={typ} />
            {:else}
                <img src={logoUrl} alt="Client Logo" onerror={() => (showDefault = true)} />
            {/if}
        </div>

        <h4>{t.authorize.login}</h4>
        <div>
            <span class="label">{t.common.email}</span>
            <div class="field">name@example.com</div>
        </div>
        <div>
            <span class="label">{t.common.password}</span>
            <div class="field focus">••••••••••</div>
        </div>
        <div class="primary">{t.authorize.login}</div>
        <div class="divider">{t.authorize.orLoginWith}</div>
        <div class="secondary">{t.authorize.passkey}</div>
    </div>

    <div aria-hidden="true" class="swatches">
        <span style:background="hsl(var(--accent))"></span>
        <span style:background="hsl(var(--error))"></span>
        <span style:background="var(--theme-sun)"></span>
        <span style:background="var(--theme-moon)"></span>
    </div>
</div>

<style>
    /* A scaled down copy of the Aurora login page from PublicPage.svelte and global.css */
    .branding-preview {
        --canvas-a: hsl(var(--action));
        --canvas-b: hsl(var(--accent));
        --card-radius: clamp(14px, calc(var(--border-radius) * 4), 26px);
        --field-radius: clamp(8px, calc(var(--border-radius) * 2), 14px);
        --outline: hsl(var(--text) / 0.18);
        --font-display: 'Bricolage Grotesque', system-ui, -apple-system, 'Segoe UI', sans-serif;
        --font-mono: 'JetBrains Mono', ui-monospace, Menlo, Consolas, monospace;

        position: relative;
        isolation: isolate;
        overflow: hidden;
        display: grid;
        justify-items: center;
        gap: 0.75rem;
        padding: 1.5rem 1rem 1rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        background: hsl(var(--bg));
        color: hsl(var(--text));
    }

    .branding-preview::before {
        content: '';
        position: absolute;
        inset: -30%;
        z-index: -2;
        pointer-events: none;
        background:
            radial-gradient(
                closest-side at 22% 26%,
                color-mix(in srgb, var(--canvas-a) 42%, transparent),
                transparent
            ),
            radial-gradient(
                closest-side at 78% 72%,
                color-mix(in srgb, var(--canvas-b) 36%, transparent),
                transparent
            );
        filter: blur(24px) saturate(1.15);
        opacity: 0.55;
    }

    .branding-preview[data-typ='dark']::before {
        opacity: 0.6;
    }

    .branding-preview::after {
        content: '';
        position: absolute;
        inset: 0;
        z-index: -1;
        pointer-events: none;
        background-image: radial-gradient(hsl(var(--text) / 0.22) 1px, transparent 1.4px);
        background-size: 16px 16px;
        mask-image: radial-gradient(ellipse 70% 60% at 50% 45%, black 10%, transparent 75%);
    }

    .card {
        position: relative;
        display: grid;
        gap: 0.65rem;
        width: 100%;
        padding: 1.1rem;
        border: 1px solid var(--outline);
        border-radius: var(--card-radius);
        background: color-mix(in srgb, hsl(var(--bg)) 82%, transparent);
        backdrop-filter: blur(14px) saturate(1.4);
        box-shadow:
            0 1px 0 hsl(var(--bg)) inset,
            0 16px 40px -20px color-mix(in srgb, var(--canvas-a) 45%, hsl(var(--text) / 0.35));
        font-size: 0.8125rem;
        line-height: 1.25rem;
    }

    .card::before {
        content: '';
        position: absolute;
        inset-inline: 1.1rem;
        top: -1px;
        height: 2px;
        border-radius: 2px;
        background: linear-gradient(
            90deg,
            transparent,
            var(--canvas-a) 20%,
            var(--canvas-b) 80%,
            transparent
        );
    }

    .logo {
        height: 2.75rem;
        display: flex;
        filter: drop-shadow(0 6px 12px color-mix(in srgb, var(--canvas-a) 35%, transparent));
    }

    .logo img {
        height: 100%;
        max-width: 100%;
        object-fit: contain;
    }

    h4 {
        margin: 0;
        color: hsl(var(--text-high));
        font-family: var(--font-display);
        font-size: 1.35rem;
        font-weight: 750;
        line-height: 1.1;
        letter-spacing: -0.03em;
    }

    .label,
    .divider {
        font-family: var(--font-mono);
        font-size: 0.625rem;
        letter-spacing: 0.12em;
        text-transform: uppercase;
    }

    .field {
        margin-top: 0.3rem;
        padding: 0.5rem 0.75rem;
        border: 1px solid var(--outline);
        border-radius: var(--field-radius);
        background: color-mix(in srgb, hsl(var(--bg)) 70%, transparent);
        color: hsl(var(--text-high));
    }

    .field.focus {
        border-color: var(--canvas-a);
        box-shadow: 0 0 0 3px color-mix(in srgb, var(--canvas-a) 22%, transparent);
    }

    .primary,
    .secondary {
        padding: 0.55rem 0.75rem;
        border-radius: var(--field-radius);
        font-weight: 600;
        text-align: center;
    }

    .primary {
        margin-top: 0.2rem;
        background: linear-gradient(
            110deg,
            var(--canvas-a) 0%,
            color-mix(in srgb, var(--canvas-a) 70%, var(--canvas-b)) 45%,
            var(--canvas-a) 55%,
            color-mix(in srgb, var(--canvas-a) 80%, var(--canvas-b)) 100%
        );
        color: var(--btn-text);
        box-shadow:
            0 1px 0 hsl(0 0% 100% / 0.25) inset,
            0 6px 16px -8px var(--canvas-a);
    }

    .secondary {
        border: 1px solid var(--outline);
        background: color-mix(in srgb, hsl(var(--bg)) 60%, transparent);
        color: hsl(var(--text-high));
    }

    .divider {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        text-align: center;
    }

    .divider::before,
    .divider::after {
        content: '';
        flex: 1;
        border-top: 1px dashed var(--outline);
    }

    .swatches {
        display: flex;
        gap: 6px;
    }

    .swatches span {
        height: 10px;
        width: 10px;
        border-radius: 999px;
    }
</style>
