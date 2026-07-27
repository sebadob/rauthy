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

<div class="container" style:height>
    <div class="edit">
        <div class="editor">
            <EditorMarkdown bind:markdown={content} withPreview={previewEmbedded} />
        </div>

        {#if !previewEmbedded}
            <div class="render">
                <i>Preview</i>
                <MarkdownRenderer />
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        max-width: calc(2 * 485pt + 1.5rem);
        display: flex;
        flex-direction: column;
        /*border: 1px solid green;*/
    }

    .edit {
        height: 100%;
        display: flex;
        flex: 1;
    }

    .editor {
        height: 100%;
        flex: 1;
        /*border: 1px solid green;*/
    }

    .render {
        padding: 0.5rem;
        flex: 1;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        overflow-y: auto;
    }

    @media (max-width: 450px) {
        .container {
            min-width: inherit;
        }
    }
</style>
