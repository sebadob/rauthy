<script lang="ts">
    import {useMarkdownWorker} from "$lib/text_edit/useWorker.svelte";

    let {
        markdown,
        show,
    }: {
        markdown: string,
        show: boolean,
    } = $props();

    let worker = useMarkdownWorker();

    $effect(() => {
        worker.renderMarkdown(markdown);
    });
</script>

{#if show}
    <div class="preview">
        {@html worker.renderedMarkdown()}
    </div>
{/if}

<style>
    .preview {
        display: block;
        flex: 1;
        padding: .5rem;
    }
</style>
