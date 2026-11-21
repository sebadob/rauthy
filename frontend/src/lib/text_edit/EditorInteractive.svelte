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
        beforeEditor,
        footer,
    }: {
        title: string;
        mode?: 'Text' | 'Markdown';
        contentRaw?: string;
        expanded?: boolean;
        fields: Snippet;
        beforeEditor?: Snippet;
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
            <span class="format-label" id={formatId}>{ta.editor.format ?? 'Format'}</span>
            <div class="format-options" role="radiogroup" aria-labelledby={formatId}>
                <label>
                    <input type="radio" name={formatId} value="Markdown" bind:group={mode} />
                    <span>Markdown</span>
                </label>
                <label>
                    <input type="radio" name={formatId} value="Text" bind:group={mode} />
                    <span>{ta.editor.plainText ?? 'Plain text'}</span>
                </label>
            </div>
        </div>
    </header>
    <div class="fields">{@render fields()}</div>
    {#if beforeEditor}
        <div class="before-editor">{@render beforeEditor()}</div>
    {/if}
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
        min-width: 0;
        overflow-wrap: anywhere;
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

    .before-editor {
        flex: none;
        --ui-field-margin: 0;
    }

    .format {
        flex: none;
        display: flex;
        align-items: center;
        gap: 0.375rem;
    }

    .format-label {
        color: var(--ui-muted);
        font-size: 0.8125rem;
    }

    .format-options {
        display: flex;
        gap: 2px;
        padding: 3px;
        background: var(--ui-container);
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
    }

    .format-options label {
        position: relative;
        display: flex;
    }

    .format-options input {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        margin: 0;
        opacity: 0;
        cursor: pointer;
    }

    .format-options label span {
        display: grid;
        place-items: center;
        min-height: 32px;
        padding: 0 0.625rem;
        border-radius: calc(var(--border-radius) - 2px);
        color: var(--ui-muted);
        font-size: 0.8125rem;
        font-weight: 600;
    }

    .format-options label:hover span {
        background: var(--ui-hover);
    }

    .format-options input:checked + span {
        background: var(--ui-surface);
        color: var(--ui-text);
        box-shadow: 0 0 0 1px var(--ui-outline);
    }

    .format-options input:focus-visible + span {
        outline: 2px solid var(--ui-primary);
        outline-offset: 1px;
    }

    @media (max-width: 480px) {
        h2 {
            font-size: 1.125rem;
            line-height: 1.25rem;
        }
        .format-label {
            position: absolute;
            clip-path: inset(50%);
            width: 1px;
            height: 1px;
            overflow: hidden;
            white-space: nowrap;
        }

        .format-options label span {
            padding: 0 0.4375rem;
            font-size: 0.75rem;
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
