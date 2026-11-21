<script lang="ts">
    import IconClipboard from '#icons/IconClipboard.svelte';
    import IconEye from '#icons/IconEye.svelte';
    import IconEyeSlash from '#icons/IconEyeSlash.svelte';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        ariaLabel = '',
        value = '',
        rows = 10,
        cols = 60,
        show = $bindable(false),
        width = 'min(32rem, 100%)',
        ...rest
    }: {
        ariaLabel: string;
        value: string;
        rows?: number;
        cols?: number;
        show?: boolean;
        width?: string;
    } = $props();

    let t = useI18n();

    const text = $derived(show ? value : '•'.repeat(value.length));

    function copyToClip() {
        navigator.clipboard.writeText(value);
    }

    function toggle() {
        show = !show;
    }
</script>

<div class="secret-field" style:width>
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

        <Button ariaLabel={t.common.copyToClip} invisible isDisabled={!value} onclick={copyToClip}>
            <IconClipboard width="1.125rem" />
            <span class="action-label">{t.common.copyToClip}</span>
        </Button>
    </div>

    <textarea
        aria-label={ariaLabel}
        class="font-mono"
        value={text}
        readonly
        tabindex={show ? undefined : -1}
        {rows}
        {cols}
        {...rest}
    ></textarea>
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

    .secret-actions {
        display: flex;
        flex-wrap: wrap;
        justify-content: end;
        gap: 0.25rem;
        padding: 0.25rem;
    }

    textarea {
        display: block;
        box-sizing: border-box;
        width: 100%;
        min-width: 0;
        margin: 0;
        padding: 0.625rem 0.75rem;
        border: 0;
        border-top: 1px solid var(--ui-outline, hsl(var(--bg-high)));
        border-radius: 0 0 var(--border-radius) var(--border-radius);
        background: var(--ui-surface, hsl(var(--bg)));
        color: var(--ui-text, hsl(var(--text)));
        font-size: 0.875rem;
        resize: vertical;
        overflow-wrap: anywhere;
    }

    textarea:focus-visible {
        outline-offset: -2px;
    }

    @media (width < 520px) {
        .action-label {
            display: none;
        }
    }
</style>
