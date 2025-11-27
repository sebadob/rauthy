<script lang="ts">
    import MarkdownRenderer from '$lib/text_edit/md/MarkdownRenderer.svelte';
    import EditorMarkdown from '$lib/text_edit/md/EditorMarkdown.svelte';

    let {
        content = $bindable(''),
        height = '50rem',
    }: {
        content: string;
        height?: string;
    } = $props();

    let innerWidth: undefined | number = $state(
        typeof window !== 'undefined' ? window.innerWidth : undefined,
    );
    let previewEmbedded = $derived(!!(innerWidth && innerWidth < 1440));
</script>

<svelte:window bind:innerWidth />

<div class="container">
    <div class="editor">
        <EditorMarkdown
            bind:markdown={content}
            withPreview={previewEmbedded}
            {height}
        />
    </div>

    {#if !previewEmbedded}
        <div class="editor">
            <MarkdownRenderer markdown={content} />
        </div>
    {/if}
</div>

<style>
    .container {
        display: flex;
        gap: 0.25rem;
    }

    .editor {
        height: min(37.8rem, 100dvh - 16rem);
        flex: 1;
        overflow: auto;
    }

    @media (max-width: 450px) {
        .container {
            min-width: inherit;
        }
    }
</style>
