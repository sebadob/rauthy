<script lang="ts">
    import type { Snippet } from 'svelte';

    let {
        checked = $bindable(false),
        disabled,
        ariaLabel = '',
        borderColor = 'hsl(var(--bg-high))',
        name,
        children,
    }: {
        checked: boolean;
        disabled?: boolean;
        ariaLabel: string;
        borderColor?: string;
        name?: string;
        children?: Snippet;
    } = $props();

    function onclick() {
        checked = !checked;
    }

    function onkeydown(ev: KeyboardEvent) {
        if (ev.code === 'Enter') {
            onclick();
        }
    }
</script>

<label class="font-label noselect admin-checkbox" data-disabled={disabled}>
    <input
        type="checkbox"
        style:border-color={borderColor}
        {name}
        bind:checked
        {disabled}
        aria-disabled={disabled}
        aria-checked={checked}
        aria-label={ariaLabel}
        {onclick}
        {onkeydown}
    />
    {#if children}<span>{@render children()}</span>{/if}
</label>

<style>
    label {
        display: grid;
        grid-template-columns: 1rem auto;
        gap: 0.66rem;
        cursor: pointer;
    }

    label[data-disabled='true'] {
        cursor: not-allowed;
    }

    input[type='checkbox'] {
        display: grid;
        place-content: center;
        margin: 0;
        appearance: none;
        width: 1.25rem;
        height: 1.25rem;
        background: hsl(var(--bg));
        border: 0.15rem solid hsl(var(--bg-high));
        border-radius: 0.2rem;
        transform: translateY(0.1rem);
        cursor: pointer;
    }

    input[type='checkbox']::before {
        content: '';
        width: 0.8rem;
        height: 0.8rem;
        border-radius: 2px;
        transform: scale(0);
        background: hsl(var(--action));
        transition: 150ms transform ease-in-out;
        transform-origin: bottom left;
        clip-path: polygon(14% 44%, 0 65%, 50% 100%, 100% 16%, 80% 0%, 43% 62%);
    }

    input[type='checkbox']:checked::before {
        transform: scale(1);
    }

    input[type='checkbox']:focus-visible {
        outline: 2px solid hsl(var(--accent));
        outline-offset: 2px;
    }

    input[type='checkbox']:disabled::before {
        background: hsl(var(--text));
    }

    input[type='checkbox']:disabled {
        cursor: not-allowed;
    }

    label > span {
        transform: translateY(0.05rem);
    }

    :global(.admin-shell) .admin-checkbox:not(:global(.branding-preview *)) {
        display: inline-flex;
        align-items: center;
        gap: 10px;
        min-height: 36px;
        margin: 0;
        font-family: inherit;
        font-size: 13px;
        font-weight: 400;
        line-height: 20px;
    }
    :global(.admin-shell) .admin-checkbox:not(:global(.branding-preview *)) :global(input) {
        flex-shrink: 0;
        width: 18px;
        height: 18px;
        margin: 0;
        padding: 0;
        border: 2px solid hsl(var(--text) / 0.6);
        border-radius: var(--border-radius);
        transform: none;
        background: transparent;
    }
    :global(.admin-shell) .admin-checkbox:not(:global(.branding-preview *)) :global(input:checked) {
        background: var(--ui-primary);
        border-color: var(--ui-primary);
    }
    :global(.admin-shell) .admin-checkbox:not(:global(.branding-preview *)) :global(input::before) {
        width: 11px;
        height: 11px;
        background: var(--ui-on-primary);
    }
    :global(.admin-shell) .admin-checkbox:not(:global(.branding-preview *)) > :global(span) {
        transform: none;
    }
    :global(.admin-shell) .admin-checkbox[data-disabled='true']:not(:global(.branding-preview *)) {
        opacity: 0.5;
    }
</style>
