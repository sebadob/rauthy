<script lang="ts">
    import Tabs from "$lib/tabs/Tabs.svelte";
    import EditorMarkdownPreview from "$lib/text_edit/md/EditorMarkdownPreview.svelte";
    import EditorText from "$lib/text_edit/EditorText.svelte";
    import {useMarkdownWorker} from "$lib/text_edit/useWorker.svelte";

    let {
        height = 'calc(90dvh - 5.5rem)',
        mode = $bindable('Markdown'),
        sanitizedValue = $bindable(''),
    }: {
        height?: string,
        mode?: 'Text' | 'Markdown' | 'HTML',
        sanitizedValue: string,
    } = $props();

    let worker = useMarkdownWorker();

    // let content = $state('');
    let content = $state(`
# head 1
## head 2
### head 3
#### head 4
##### head 5
###### head 6

- list 1
- list 2
- ...

1. erstens
2. zweitens
3. ...

with some \`code\` ?
and so on...

#### head 2

Und Multiline code?

\`\`\`
multi line
code block
how does it look?
\`\`\`
    `);

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
    <Tabs bind:selected={mode} tabs={['Markdown', 'Text']} center/>
</div>

{#if mode === 'Markdown'}
    <EditorMarkdownPreview bind:content {height}/>
    <!--{:else if mode === 'HTML'}-->
    <!--    <EditorHtml bind:content {height}/>-->
{:else if mode === 'Text'}
    <EditorText bind:content {height} hideButtons/>
{/if}

<style>
    .mode {
        margin: .5rem 0;
        width: 11rem;
    }
</style>
