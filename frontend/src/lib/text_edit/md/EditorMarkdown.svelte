<script lang="ts">
    import ToolbarIcon from '#lib/text_edit/md/ToolbarIconMarkdown.svelte';
    import IconBold from '#icons/editor/IconBold.svelte';
    import IconH1 from '#icons/editor/IconH1.svelte';
    import IconH2 from '#icons/editor/IconH2.svelte';
    import IconH3 from '#icons/editor/IconH3.svelte';
    import IconItalic from '#icons/editor/IconItalic.svelte';
    import IconStrikeThrough from '#icons/editor/IconStrikeThrough.svelte';
    import IconListBullet from '#icons/editor/IconListBullet.svelte';
    import IconListNumbered from '#icons/editor/IconListNumbered.svelte';
    import IconQueueList from '#icons/editor/IconQueueList.svelte';
    import IconCode from '#icons/editor/IconCode.svelte';
    import IconQuote from '#icons/editor/IconQuote.svelte';
    import IconLink from '#icons/editor/IconLink.svelte';
    import Button from '#lib/button/Button.svelte';
    import MarkdownRenderer from '#lib/text_edit/md/MarkdownRenderer.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        markdown = $bindable(),
        height,
    }: {
        markdown: string;
        height: string;
    } = $props();

    const iconSize = '1.125rem';
    let ta = useI18nAdmin();
    let t = useI18n();
    let ref: undefined | HTMLDivElement = $state();
    let preview = $state(false);
    let toolbar: HTMLDivElement;

    function onkeydown(event: KeyboardEvent) {
        if (
            !(event.ctrlKey || event.metaKey) ||
            event.altKey ||
            event.shiftKey ||
            event.isComposing
        ) {
            return;
        }
        const key = event.key.toLowerCase();
        if (key === 'b' || key === 'i' || key === 'k') {
            event.preventDefault();
            toolbar.querySelector(`[data-shortcut="${key}"]`)?.closest('button')?.click();
        }
    }
</script>

<div class="editor" style:height>
    <div class="toolbar" bind:this={toolbar}>
        <div class="view" role="group" aria-label="Markdown">
            <Button invisible ariaPressed={!preview} onclick={() => (preview = false)}>
                {t.common.edit}
            </Button>
            <Button invisible ariaPressed={preview} onclick={() => (preview = true)}>
                {ta.common.preview}
            </Button>
        </div>

        {#if !preview}
            <div class="actions">
                <ToolbarIcon {ref} title={ta.editor.heading1} bind:content={markdown} addLeft="# ">
                    <IconH1 width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon {ref} title={ta.editor.heading2} bind:content={markdown} addLeft="## ">
                    <IconH2 width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon
                    {ref}
                    title={ta.editor.heading3}
                    bind:content={markdown}
                    addLeft="### "
                >
                    <IconH3 width={iconSize} />
                </ToolbarIcon>

                <ToolbarIcon
                    {ref}
                    title={ta.editor.bold}
                    shortcut="b"
                    bind:content={markdown}
                    addLeft="**"
                    addRight="**"
                >
                    <IconBold width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon
                    {ref}
                    title={ta.editor.italic}
                    shortcut="i"
                    bind:content={markdown}
                    addLeft="*"
                    addRight="*"
                >
                    <IconItalic width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon
                    {ref}
                    title={ta.editor.strikeThrough}
                    bind:content={markdown}
                    addLeft="~~"
                    addRight="~~"
                >
                    <IconStrikeThrough width={iconSize} />
                </ToolbarIcon>

                <ToolbarIcon
                    {ref}
                    title={ta.editor.listBullet}
                    bind:content={markdown}
                    addLeft="- "
                >
                    <IconListBullet width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon
                    {ref}
                    title={ta.editor.listNumbered}
                    bind:content={markdown}
                    addLeft="1. "
                >
                    <IconListNumbered width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon
                    {ref}
                    title={ta.editor.listTasks}
                    bind:content={markdown}
                    addLeft="- [ ] "
                >
                    <IconQueueList width={iconSize} />
                </ToolbarIcon>

                <ToolbarIcon
                    {ref}
                    title={ta.editor.code}
                    bind:content={markdown}
                    addLeft="`"
                    addRight="`"
                >
                    <IconCode width={iconSize} />
                </ToolbarIcon>
                <ToolbarIcon {ref} title={ta.editor.quote} bind:content={markdown} addLeft="> ">
                    <IconQuote width={iconSize} />
                </ToolbarIcon>

                <!-- TODO needs a custom way of focussing via offset -->
                <ToolbarIcon
                    {ref}
                    title={ta.editor.link}
                    shortcut="k"
                    bind:content={markdown}
                    addLeft="["
                    addRightLink="https://..."
                >
                    <IconLink width={iconSize} />
                </ToolbarIcon>
            </div>
        {/if}
    </div>

    {#if preview}
        <div class="renderer">
            <MarkdownRenderer {markdown} />
        </div>
    {:else}
        <div
            role="textbox"
            tabindex="0"
            aria-multiline="true"
            bind:this={ref}
            class="edit font-mono"
            aria-label={ta.editor.textArea}
            bind:innerText={markdown}
            contenteditable="plaintext-only"
            spellcheck="false"
            {onkeydown}
        ></div>
    {/if}
</div>

<style>
    .editor {
        display: flex;
        flex-direction: column;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        min-width: 0;
    }

    .toolbar {
        display: flex;
        flex: none;
        align-items: center;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: 4px 12px;
        padding: 4px 8px;
        border-bottom: 1px solid var(--ui-outline);
        background: var(--ui-container);
    }

    .view,
    .actions {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 2px;
    }

    .toolbar {
        --ui-button-min-height: 32px;
        --ui-button-height: 32px;
        --ui-button-padding: 0 8px;
    }

    .view :global(.admin-control) {
        min-width: 4rem;
        font-size: var(--ui-control-font-size);
        font-weight: 600;
    }

    .toolbar .view :global(.admin-control[aria-pressed='true'][data-variant]) {
        background: var(--ui-tonal);
        border-color: var(--ui-primary);
        color: var(--ui-primary);
    }

    .actions {
        flex-wrap: nowrap;
        min-width: 0;
        max-width: 100%;
        overflow-x: auto;
        padding: 4px;
    }

    .actions :global(button) {
        width: 32px;
        min-width: 32px;
        padding: 0;
    }

    .edit,
    .renderer {
        flex: 1;
        min-width: 0;
        min-height: 0;
        overflow: auto;
        padding: 0.75rem;
        overflow-wrap: anywhere;
    }

    .edit {
        white-space: pre-wrap;
        outline-offset: -2px;
    }
</style>
