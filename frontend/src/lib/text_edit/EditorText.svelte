<script lang="ts">
    import type { Snippet } from 'svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';

    let {
        content = $bindable(''),
        ref = $bindable(),
        hideButtons = false,
        height = '40rem',
        // append a custom, non-editable text at the bottom
        appendCustomText,
        onCancel,
        onSave,
        onKeyUp,
        onMoueUp,
        onFocus,
        onFocusOut,
        onScroll,
        children,
    }: {
        content: string;
        ref?: undefined | HTMLElement;
        hideButtons?: boolean;
        height?: string;
        appendCustomText?: string;
        onCancel?: () => void;
        onSave?: () => void;
        onKeyUp?: () => void;
        onMoueUp?: () => void;
        onFocus?: () => void;
        onFocusOut?: () => void;
        onScroll?: () => void;
        children?: Snippet;
    } = $props();

    let t = useI18n();
</script>

<div role="none" class="wrapper" style:height onclick={() => ref?.focus()}>
    {@render children?.()}

    <div
        bind:this={ref}
        id="editor"
        role="textbox"
        tabindex="0"
        class="edit"
        contenteditable="plaintext-only"
        bind:innerText={content}
        onkeyup={onKeyUp}
        onmouseup={onMoueUp}
        onfocus={onFocus}
        onfocusout={onFocusOut}
        onscroll={onScroll}
    ></div>

    {#if appendCustomText}
        <div class="custom">
            <pre class="font-default">{appendCustomText}</pre>
        </div>
    {/if}

    {#if !hideButtons}
        <Button onclick={onSave}>
            {t.common.save}
        </Button>
        <Button level={3} onclick={onCancel}>
            {t.common.cancel}
        </Button>
    {/if}
</div>

<style>
    pre {
        text-wrap: wrap;
        word-break: break-word;
    }

    .custom {
        padding: 0 0.5rem 0.5rem 0.5rem;
        /*border: 1px solid red;*/
    }

    .edit {
        width: 100%;
        padding: 0.5rem;
    }

    .edit:focus {
        outline: none;
    }

    /*.edit:focus-visible {*/
    /*    outline: 1px solid hsl(var(--accent));*/
    /*}*/

    .wrapper {
        margin: 0.15rem 0;
        display: flex;
        flex-direction: column;
        border: 1px solid hsla(var(--bg-high) / 0.5);
        border-radius: var(--border-radius);
        overflow-y: auto;
        cursor: text;
    }
</style>
