<script lang="ts">
    import IconClipboard from '#icons/IconClipboard.svelte';
    import IconEye from '#icons/IconEye.svelte';
    import IconEyeSlash from '#icons/IconEyeSlash.svelte';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        ariaLabel = '',
        language,
        value = '',
        rows = 10,
        show = $bindable(false),
        width = 'min(32rem, 100%)',
    }: {
        ariaLabel: string;
        language?: string;
        value: string;
        rows?: number;
        show?: boolean;
        width?: string;
    } = $props();

    let t = useI18n();

    const text = $derived(show ? value : value.replace(/[^\n]/g, '•'));

    function copyToClip() {
        navigator.clipboard.writeText(value);
    }

    function toggle() {
        show = !show;
    }
</script>

<div class="secret-field" role="group" aria-label={ariaLabel} style:width>
    <div class="secret-header">
        <span class="secret-caption" class:language aria-hidden="true">
            {language ?? ariaLabel}
        </span>
        <div class="secret-actions">
            <Button
                ariaLabel={show ? t.common.hide : t.common.show}
                invisible
                isDisabled={!value}
                onclick={toggle}
            >
                {#if show}
                    <IconEye width="1.125rem" />
                {:else}
                    <IconEyeSlash width="1.125rem" />
                {/if}
                <span class="action-label">{show ? t.common.hide : t.common.show}</span>
            </Button>

            <Button
                ariaLabel={t.common.copyToClip}
                invisible
                isDisabled={!value}
                onclick={copyToClip}
            >
                <IconClipboard width="1.125rem" />
                <span class="action-label">{t.common.copyToClip}</span>
            </Button>
        </div>
    </div>

    <pre class="secret-value" class:masked={!show} style:--visible-lines={rows}><code>{text}</code
        ></pre>
</div>

<style>
    .secret-field {
        max-width: 100%;
        min-width: 0;
        display: grid;
        grid-template-columns: minmax(0, 1fr);
        border: 1px solid var(--ui-outline, hsl(var(--bg-high)));
        border-radius: var(--border-radius);
        background: var(--ui-container, hsl(var(--bg-high)));
    }

    .secret-header {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: space-between;
        gap: 0.25rem;
        padding: 0.25rem 0.375rem 0.25rem 0.75rem;
    }

    .secret-caption {
        color: var(--ui-muted, hsl(var(--text)));
        font-size: 0.75rem;
        font-weight: 600;
    }

    .secret-caption.language {
        font-family: monospace;
    }

    .secret-actions {
        display: flex;
        flex-wrap: wrap;
        gap: 0.25rem;
        margin-inline-start: auto;
    }

    .secret-field .secret-value {
        box-sizing: border-box;
        min-width: 0;
        min-height: calc(1.5em * var(--visible-lines) + 1.25rem);
        margin: 0;
        padding: 0.625rem 0.75rem;
        border-top: 1px solid var(--ui-outline, hsl(var(--bg-high)));
        border-radius: 0 0 var(--border-radius) var(--border-radius);
        background: color-mix(in srgb, var(--ui-surface, hsl(var(--bg))) 97%, currentColor);
        color: var(--ui-text, hsl(var(--text)));
        font-size: 0.875rem;
        line-height: 1.5;
        white-space: pre-wrap;
        overflow-wrap: anywhere;
    }

    .secret-value.masked {
        color: var(--ui-muted, hsl(var(--text)));
    }

    .secret-value code {
        padding: 0;
        background: none;
        font-size: inherit;
    }

    @media (width < 520px) {
        .action-label {
            display: none;
        }
    }
</style>
