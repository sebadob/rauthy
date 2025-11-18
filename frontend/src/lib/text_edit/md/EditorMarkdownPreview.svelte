<script lang="ts">
    import MarkdownRenderer from "$lib/text_edit/md/MarkdownRenderer.svelte";
    import Button from "$lib/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import EditorMarkdown from "$lib/text_edit/md/EditorMarkdown.svelte";

    let {
        content = $bindable(''),
        onCancel,
        onSave,
        hideButtons = false,
        height = '50rem',
    }: {
        content: string,
        onCancel?: () => void,
        onSave?: () => void,
        hideButtons?: boolean,
        height?: string,
    } = $props();

    let t = useI18n();

    let innerWidth: undefined | number = $state(typeof window !== 'undefined' ? window.innerWidth : undefined);

    let previewEmbedded = $derived(!!(innerWidth && innerWidth < 1440));
    $inspect(previewEmbedded, 'previewEmbedded');

</script>

<svelte:window bind:innerWidth/>

<div class="container">
    <div class="editor">
        <EditorMarkdown
                bind:markdown={content}
                withPreview={previewEmbedded}
                {height}
        />
    </div>

    <MarkdownRenderer
            markdown={content}
            show={!previewEmbedded}
    />
</div>

{#if !hideButtons}
    <div class="space"></div>

    <Button onclick={onSave}>
        {t.common.save}
    </Button>
    <Button level={3} onclick={onCancel}>
        {t.common.cancel}
    </Button>
{/if}

<style>
    .container {
        display: flex;
        gap: .25rem;
    }

    .editor {
        height: 100%;
        flex: 1;
    }

    @media (max-width: 450px) {
        .container {
            min-width: inherit;
        }
    }

    .space {
        height: 1rem;
    }
</style>
