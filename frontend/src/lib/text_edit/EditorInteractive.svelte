<script lang="ts">
    import Tabs from '$lib/tabs/Tabs.svelte';
    import EditorMarkdownPreview from '$lib/text_edit/md/EditorMarkdownPreview.svelte';
    import EditorText from '$lib/text_edit/EditorText.svelte';
    import { useMarkdownWorker } from '$lib/text_edit/useWorker.svelte';
    import { onDestroy } from 'svelte';

    let {
        height = 'calc(90dvh - 5.5rem)',
        mode = $bindable('Markdown'),
        sanitizedValue = $bindable(''),
    }: {
        height?: string;
        mode?: 'Text' | 'Markdown' | 'HTML';
        sanitizedValue: string;
    } = $props();

    let worker = useMarkdownWorker();
    let content = $state('');

    onDestroy(() => {
        worker.closeWorker();
    });

    $effect(() => {
        if (mode === 'Text') {
            sanitizedValue = content;
        } else if (mode === 'Markdown') {
            sanitizedValue = worker.renderedMarkdown();
            // } else if (mode === 'HTML') {
            //     sanitizedValue = worker.sanitizedHTML();
        }
    });
</script>

<div class="mode">
    <Tabs
        bind:selected={mode}
        tabs={['Markdown', 'Text']}
        center
    />
</div>

{#if mode === 'Markdown'}
    <EditorMarkdownPreview
        bind:content
        {height}
    />
    <!--{:else if mode === 'HTML'}-->
    <!--    <EditorHtml bind:content {height}/>-->
{:else if mode === 'Text'}
    <EditorText
        bind:content
        {height}
        hideButtons
    />
{/if}

<style>
    .mode {
        margin: 0.5rem 0;
        width: 11rem;
    }
</style>
