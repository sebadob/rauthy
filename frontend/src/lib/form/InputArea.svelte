<script lang="ts">
    import { slide } from 'svelte/transition';
    import { untrack } from 'svelte';
    import { genKey } from '#utils/helpers.ts';

    let {
        ref = $bindable(),
        id,
        name,
        value = $bindable(''),
        label = '',

        rows = 3,
        placeholder = '',
        formatBadge,
        errMsg = '',
        disabled = false,
        fontMono = false,

        maxLength,
        required = false,
        pattern,
        isError = $bindable(false),

        width = 'inherit',

        onBlur,
        onLeft,
        onRight,
        onUp,
        onDown,
    }: {
        ref?: undefined | HTMLTextAreaElement;
        id?: string;
        name?: string;
        value?: string;
        label?: string;
        rows?: number;
        placeholder?: string;
        formatBadge?: string;
        errMsg?: string;
        disabled?: boolean | null | undefined;
        fontMono?: boolean | null | undefined;
        maxLength?: number | null | undefined;
        step?: number;
        required?: boolean;
        pattern?: string;
        isError?: boolean;
        width?: string;

        onBlur?: () => void;
        onLeft?: () => void;
        onRight?: () => void;
        onUp?: () => void;
        onDown?: () => void;
    } = $props();

    const re = untrack(() => (pattern ? new RegExp(pattern, 'gm') : undefined));

    function onblur(
        event: FocusEvent & {
            currentTarget: EventTarget & HTMLTextAreaElement;
        },
    ) {
        isValid();
        onBlur?.();
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        isValid();
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        event.preventDefault();
        isError = true;
    }

    function onsubmit(event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        if (!isValid()) {
            event.preventDefault();
        }
    }

    function onkeydown(ev: KeyboardEvent) {
        isValid();
        switch (ev.code) {
            case 'ArrowUp':
                onUp?.();
                break;
            case 'ArrowDown':
                onDown?.();
                break;
            case 'ArrowLeft':
                onLeft?.();
                break;
            case 'ArrowRight':
                onRight?.();
                break;
        }
    }

    function isValid() {
        if (re) {
            const matches = value.match(re);
            if (matches && matches.length > 2) {
                ref?.setCustomValidity(errMsg);
                isError = true;
            } else {
                ref?.setCustomValidity('');
                isError = false;
            }
        }
        return true;
    }
    const badgeId = genKey();
</script>

<div class="container admin-textarea-field" style:width>
    <div class="label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
    </div>
    <div class="area" class:with-badge={formatBadge}>
        <textarea
            bind:this={ref}
            {id}
            aria-describedby={formatBadge ? badgeId : undefined}
            {name}
            class:font-mono={fontMono}
            title={errMsg}
            aria-label={label || placeholder}
            bind:value
            {rows}
            autocomplete="off"
            {placeholder}
            aria-placeholder={placeholder}
            {disabled}
            aria-disabled={disabled}
            maxlength={maxLength || undefined}
            required={required || undefined}
            aria-required={required || false}
            aria-invalid={isError}
            {onsubmit}
            {oninput}
            {oninvalid}
            {onblur}
            {onkeydown}
        >
        </textarea>
        {#if formatBadge}<span class="ui-format-badge" id={badgeId}>{formatBadge}</span>{/if}
    </div>
    {#if isError}
        <div class="error" transition:slide={{ duration: 150 }}>
            {errMsg}
        </div>
    {/if}
</div>

<style>
    .area {
        display: contents;
    }

    .with-badge {
        display: grid;
        position: relative;
    }

    :global(.ui-shell) .with-badge textarea:not(:global(.branding-preview *)) {
        padding-block-start: 2rem;
    }

    .with-badge :global(.ui-format-badge) {
        inset-block-start: 0.5rem;
        transform: none;
    }

    label {
        font-size: 0.9em;
        flex-wrap: wrap;
    }

    textarea {
        width: 100%;
        padding: 0.25rem 0.5rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        color: hsl(var(--text));
        background: transparent;
        outline: none;
        resize: none;
    }

    textarea:invalid {
        border: 1px solid hsl(var(--error));
    }

    textarea:focus-visible {
        outline: hsl(var(--accent));
    }

    .container {
        margin: 0.5rem 0;
    }

    .error {
        color: hsl(var(--error));
        margin-top: -0.4rem;
        padding-left: 0.1rem;
        font-size: 0.8rem;
    }

    .label {
        width: 100%;
        padding-left: 0.1rem;
        padding-top: 0.1rem;
    }

    :global(.ui-shell) .label:not(:global(.branding-preview *)) {
        padding: 0;
        margin: 0 0 6px;
        line-height: 18px;
    }
    :global(.ui-shell) textarea:not(:global(.branding-preview *)) {
        box-sizing: border-box;
        width: 100%;
        min-width: 0;
        min-height: 44px;
        margin: 0;
        padding: 11px 14px;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        color: var(--ui-text);
        font: inherit;
        font-size: 14px;
        line-height: 20px;
        box-shadow: none;
    }
    :global(.ui-shell) textarea:not(:global(.branding-preview *))::placeholder {
        color: hsl(var(--text) / 0.65);
    }
    :global(.ui-shell) textarea:focus:not(:global(.branding-preview *)) {
        outline: 2px solid var(--ui-primary);
        outline-offset: -1px;
    }

    :global(.ui-shell) .admin-textarea-field:not(:global(.branding-preview *)) {
        max-width: 100%;
        min-width: 0;
    }
    :global(.ui-shell) textarea:not(:global(.branding-preview *)) {
        max-width: 100%;
        resize: vertical;
    }
</style>
