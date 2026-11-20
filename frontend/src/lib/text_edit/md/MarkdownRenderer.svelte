<script lang="ts">
    import { useMarkdownWorker } from '#lib/text_edit/useWorker.svelte.js';

    let {
        markdown,
    }: {
        markdown: string;
    } = $props();

    let worker = useMarkdownWorker();

    $effect(() => {
        worker.renderMarkdown(markdown);
    });
</script>

<div class="preview html-render">
    {@html worker.renderedMarkdown()}
</div>

<style>
    .preview {
        /*max-width: 467pt;*/
        display: block;
        padding: 0.5rem;
        overflow-wrap: anywhere;
        overflow: auto;
    }
</style>
