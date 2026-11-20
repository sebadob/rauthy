<script lang="ts">
    import type { Snippet } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        content = $bindable(''),
        ref = $bindable(),
        hideButtons = false,
        ariaLabel,
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
        ariaLabel?: string;
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

<div role="none" class="wrapper" style:min-height={height} onclick={() => ref?.focus()}>
    {@render children?.()}

    <div
        bind:this={ref}
        id="editor"
        role="textbox"
        aria-label={ariaLabel}
        aria-multiline="true"
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
        flex: 1;
        padding: 0.75rem;
        overflow-wrap: anywhere;
        white-space: pre-wrap;
    }

    .wrapper {
        display: flex;
        flex-direction: column;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        cursor: text;
    }
</style>
