<script lang="ts">
    import Tabs from '#lib/tabs/Tabs.svelte';
    import EditorMarkdown from '#lib/text_edit/md/EditorMarkdown.svelte';
    import EditorText from '#lib/text_edit/EditorText.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        height = '16rem',
        mode = $bindable('Markdown'),
        contentRaw = $bindable(''),
    }: {
        height?: string;
        mode?: 'Text' | 'Markdown';
        contentRaw?: string;
    } = $props();

    const ta = useI18nAdmin();
</script>

<div class="mode">
    <Tabs bind:selected={mode} tabs={['Markdown', 'Text']} center />
</div>

{#if mode === 'Markdown'}
    <EditorMarkdown bind:markdown={contentRaw} {height} />
{:else}
    <EditorText bind:content={contentRaw} {height} ariaLabel={ta.editor.textArea} hideButtons />
{/if}

<style>
    .mode {
        --ui-tabs-margin: 0;
        margin: 0.5rem 0 0.75rem;
        width: 11rem;
    }
</style>
