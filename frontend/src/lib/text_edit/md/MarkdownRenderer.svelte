<script lang="ts">
    import { useMarkdownWorker } from '$lib/text_edit/useWorker.svelte';

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
        word-break: break-all;
        overflow: auto;
    }
</style>
