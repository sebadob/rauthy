<script lang="ts">
    import type { Snippet } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import EditorMarkdown from '#lib/text_edit/md/EditorMarkdown.svelte';
    import EditorText from '#lib/text_edit/EditorText.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        title,
        mode = $bindable('Markdown'),
        contentRaw = $bindable(''),
        expanded = $bindable(false),
        fields,
        footer,
    }: {
        title: string;
        mode?: 'Text' | 'Markdown';
        contentRaw?: string;
        expanded?: boolean;
        fields: Snippet;
        footer: Snippet;
    } = $props();

    const t = useI18n();
    const ta = useI18nAdmin();
    const formatId = $props.id();
</script>

<div class="composer" class:expanded>
    <header>
        <h2>{title}</h2>
        <div class="format">
            <label for={formatId}>{ta.editor.format ?? 'Format'}</label>
            <select id={formatId} class="ui-text-input" bind:value={mode}>
                <option value="Markdown">Markdown</option>
                <option value="Text">{ta.editor.plainText ?? 'Plain text'}</option>
            </select>
        </div>
    </header>
    <div class="fields">{@render fields()}</div>
    <div class="body">
        {#if mode === 'Markdown'}
            <EditorMarkdown bind:markdown={contentRaw} height="100%" />
        {:else}
            <EditorText
                bind:content={contentRaw}
                height="100%"
                ariaLabel={ta.editor.textArea}
                hideButtons
            />
        {/if}
    </div>
    <div class="footer">
        <div class="footer-content">{@render footer()}</div>
        <Button
            invisible
            ariaLabel={t.common.expandContent}
            ariaPressed={expanded}
            onclick={() => (expanded = !expanded)}
        >
            <span title={t.common.expandContent}>
                <svg
                    aria-hidden="true"
                    width="18"
                    height="18"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    {#if expanded}
                        <path d="M3 8h5V3m13 5h-5V3M3 16h5v5m13-5h-5v5" />
                    {:else}
                        <path d="M8 3H3v5m13-5h5v5M8 21H3v-5m13 5h5v-5" />
                    {/if}
                </svg>
            </span>
        </Button>
    </div>
</div>

<style>
    .composer {
        --composer-width: calc(100dvw - 34px - 2 * var(--dialog-padding, 32px));
        --composer-height: max(12rem, calc(100dvh - 34px - 2 * var(--dialog-padding, 32px)));
        display: flex;
        flex-direction: column;
        width: min(54rem, var(--composer-width));
        min-width: 0;
        max-height: var(--composer-height);
        gap: 0.5rem;
    }

    .composer.expanded {
        --composer-width: calc(100dvw - 2 * var(--dialog-padding, 32px));
        --composer-height: max(12rem, calc(100dvh - 2 * var(--dialog-padding, 32px)));
        width: var(--composer-width);
        height: var(--composer-height);
    }

    header {
        display: flex;
        flex: none;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
        padding-inline-end: 1.75rem;
    }

    h2 {
        margin: 0;
        color: var(--ui-text);
        font-size: 1.375rem;
        line-height: 1.75rem;
    }

    .footer :global(button[aria-pressed='true']) {
        background: var(--ui-tonal);
        color: var(--ui-primary);
    }

    .fields {
        min-height: min(2.5rem, 15dvh);
        margin-inline: -4px;
        padding: 1px 4px;
        overflow: auto;
        scrollbar-width: none;
    }

    .fields::-webkit-scrollbar {
        display: none;
    }

    .format {
        flex: none;
        display: flex;
        align-items: center;
        gap: 0.375rem;
    }

    .format label {
        color: var(--ui-muted);
        font-size: 0.8125rem;
    }

    .format select {
        width: auto;
        min-height: 32px;
        padding: 4px 8px;
    }

    @media (max-width: 480px) {
        h2 {
            font-size: 1.125rem;
            line-height: 1.25rem;
        }
    }

    @media (max-width: 359px) {
        .format {
            flex-direction: column;
            align-items: flex-start;
            gap: 2px;
        }
    }

    .body {
        flex: 0 1 clamp(16rem, 43dvh, 32rem);
        min-height: min(9rem, 35dvh);
        min-width: 0;
    }

    @media (max-width: 600px) {
        .body {
            flex-basis: clamp(14rem, 38dvh, 24rem);
            flex-shrink: 3;
        }
    }

    @media (max-width: 600px) and (min-height: 620px) {
        .fields {
            flex-shrink: 0;
            max-height: 36dvh;
        }
    }

    .expanded .body {
        flex: 1 1 auto;
    }

    .footer {
        flex: none;
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        gap: 0.5rem;
        padding-top: 0.5rem;
        border-top: 1px solid var(--ui-outline);
    }

    .footer-content {
        flex: 1;
        min-width: 0;
    }
</style>
