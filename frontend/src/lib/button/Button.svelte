<script lang="ts">
    import { fade } from 'svelte/transition';
    import { getContext, type Snippet, untrack } from 'svelte';
    import Loading from '#lib/Loading.svelte';

    let {
        type = 'button',
        role = 'button',
        ref = $bindable(),
        id,
        ariaLabel,
        ariaControls,
        ariaExpanded,
        ariaCurrent,
        level = 1,
        width,
        isDisabled = false,
        isLoading = false,
        invisible = false,
        invisibleOutline = false,
        popovertarget,
        popovertargetaction,
        onclick,
        onTab,
        onLeft,
        onRight,
        onUp,
        onDown,
        children,
        ...rest
    }: {
        type?: 'button' | 'submit' | 'reset' | null | undefined;
        role?: string;
        ref?: undefined | HTMLButtonElement;
        id?: string;
        ariaLabel?: string;
        ariaControls?: string;
        ariaExpanded?: boolean;
        ariaCurrent?: 'time' | 'page' | 'step' | 'location' | 'date' | undefined;
        level?: number;
        width?: string;
        isDisabled?: boolean;
        isLoading?: boolean;
        invisible?: boolean;
        invisibleOutline?: boolean;
        popovertarget?: string;
        popovertargetaction?: 'toggle' | 'show' | 'hide' | null | undefined;
        children: Snippet;

        onclick?: (ev: Event) => void;
        onTab?: () => void;
        onLeft?: () => void;
        onRight?: () => void;
        onUp?: () => void;
        onDown?: () => void;
        // passed on to the <button> with the other remaining props
        onmousedown?: (ev: MouseEvent) => void;
    } = $props();

    const adminUi = getContext<boolean>('rauthy-admin-ui') || false;

    let cls = $derived.by(() => {
        if (invisible) {
            return 'invisible';
        }
        switch (level) {
            case -1:
                return 'l1d';
            case -2:
                return 'l2d';
            case -3:
                return 'l3d';
            case 2:
                return 'l2';
            case 3:
                return 'l3';
            default:
                return 'l1';
        }
    });
    let showText = $state(untrack(() => !isLoading));
    let disabled = $derived(isDisabled || isLoading);

    $effect(() => {
        if (isLoading) {
            setTimeout(() => {
                showText = false;
            }, 120);
        } else {
            setTimeout(() => {
                showText = true;
            }, 120);
        }
    });

    function loadingColor() {
        switch (level) {
            case -1:
                return 'var(--btn-text)';
            case -2:
                return 'hsl(var(--error))';
            case -3:
                return 'hsl(var(--error))';
            case 2:
                return 'hsl(var(--action))';
            case 3:
                return 'hsl(var(--action))';
            default:
                return 'var(--btn-text)';
        }
    }

    function onkeydown(ev: KeyboardEvent) {
        switch (ev.code) {
            case 'Enter':
                if (onclick) {
                    ev.preventDefault();
                    onclick(ev);
                }
                break;
            case 'ArrowLeft':
                onLeft?.();
                break;
            case 'ArrowRight':
                onRight?.();
                break;
            case 'ArrowUp':
                onUp?.();
                break;
            case 'ArrowDown':
                onDown?.();
                break;
            case 'Tab':
                if (onTab) {
                    onTab();
                    ev.preventDefault();
                }
                break;
        }
    }
</script>

<button
    bind:this={ref}
    name={ariaLabel}
    {role}
    {type}
    {id}
    aria-label={ariaLabel}
    aria-controls={ariaControls}
    aria-expanded={ariaExpanded}
    aria-current={ariaCurrent}
    class={adminUi ? 'admin-control' : cls}
    data-variant={adminUi ? cls : undefined}
    class:invisibleOutline
    style:width
    data-isloading={isLoading}
    {onclick}
    {onkeydown}
    {disabled}
    aria-disabled={disabled}
    {popovertarget}
    {popovertargetaction}
    {...rest}
>
    {#if isLoading}
        <div class="load">
            <Loading background={false} color={loadingColor()} />
        </div>
    {:else if showText}
        <div in:fade class="children font-label">
            {@render children()}
        </div>
    {/if}
</button>

<style>
    button {
        height: 2rem;
        padding: 0 0.5rem;
        font-weight: bold;
        font-size: 0.9rem;
        outline: none;
        border: none;
        border-radius: var(--border-radius);
        cursor: pointer;
        transition: all 150ms;
    }

    button:hover {
        box-shadow: 1px 1px 2px hsl(var(--bg-high));
    }

    button:focus-visible {
        outline: 2px solid hsl(var(--accent));
    }

    .children {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .invisible,
    .invisible:hover {
        height: inherit;
        margin: 0;
        padding: 0;
        outline: none;
        background: none;
        box-shadow: none;
        font-size: 1rem;
        color: hsla(var(--action) / 0.93);
    }

    .invisible > div {
        color: hsl(var(--text));
        transition: color 150ms;
    }

    .invisible:hover > div {
        color: hsl(var(--action));
    }

    .invisibleOutline:focus {
        outline: none;
    }

    button[aria-disabled='true'],
    button[aria-disabled='true']:hover,
    button[aria-disabled='true']:focus {
        color: hsl(var(--bg-high));
    }

    button[aria-disabled='true'],
    button[data-isloading='true'] {
        cursor: not-allowed;
    }

    .l1,
    .l2,
    .l3 {
        border: 1px solid hsla(var(--action) / 0.5);
    }

    .l1 {
        color: var(--btn-text);
        background: hsla(var(--action) / 0.93);
    }

    .l1:hover {
        background: hsl(var(--action));
    }

    .l1[aria-disabled='true'] {
        background: hsla(var(--text) / 0.5);
    }

    .l2 {
        color: hsl(var(--action));
        background: transparent;
    }

    .l3 {
        color: hsla(var(--action) / 0.9);
        border: none;
        background: transparent;
    }

    .l3:hover {
        color: hsl(var(--action));
    }

    .l1d,
    .l2d,
    .l3d {
        border: 1px solid hsla(var(--error) / 0.5);
    }

    .l1d {
        color: var(--btn-text);
        background: hsla(var(--error) / 0.93);
    }

    .l1d:hover {
        background: hsl(var(--error));
    }

    .l2d {
        color: hsl(var(--error));
        border: 1px solid hsl(var(--error));
        background: transparent;
    }

    .l3d {
        color: hsl(var(--error));
        border: none;
        background: transparent;
    }

    .load {
        display: flex;
        justify-content: center;
    }

    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) {
        appearance: none;
        box-sizing: border-box;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        vertical-align: middle;
        height: 40px;
        min-height: 40px;
        min-width: 40px;
        margin: 0;
        padding: 0 20px;
        border: 1px solid transparent;
        border-radius: var(--border-radius);
        background: transparent;
        color: var(--ui-primary);
        font-family: inherit;
        font-size: 14px;
        font-weight: 600;
        line-height: 20px;
        letter-spacing: 0;
        text-align: center;
        text-decoration: none;
        cursor: pointer;
        box-shadow: none;
        transform: none;
        transition:
            background-color 120ms,
            border-color 120ms,
            color 120ms;
    }
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) > :global(.children) {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        min-width: 0;
        height: 100%;
        padding: 0;
        margin: 0;
        color: inherit;
        font: inherit;
        line-height: 20px;
        transform: none;
    }
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) :global(svg) {
        display: block;
        flex-shrink: 0;
    }
    :global(.admin-shell) .admin-control[data-variant='l1']:not(:global(.branding-preview *)) {
        background: var(--ui-primary);
        color: var(--ui-on-primary);
    }
    :global(.admin-shell) .admin-control[data-variant='l2']:not(:global(.branding-preview *)) {
        border-color: var(--ui-outline);
        background: transparent;
        color: var(--ui-primary);
    }
    :global(.admin-shell) .admin-control[data-variant='l1d']:not(:global(.branding-preview *)) {
        color: var(--ui-on-primary);
        background: hsl(var(--error));
    }
    :global(.admin-shell) .admin-control[data-variant='l2d']:not(:global(.branding-preview *)),
    :global(.admin-shell) .admin-control[data-variant='l3d']:not(:global(.branding-preview *)) {
        color: hsl(var(--error));
    }
    :global(.admin-shell) .admin-control[data-variant='l2d']:not(:global(.branding-preview *)) {
        border-color: hsl(var(--error));
    }
    :global(.admin-shell)
        .admin-control[data-variant='invisible']:not(:global(.branding-preview *)) {
        padding: var(--ui-button-padding, 0 8px);
        min-width: 36px;
        height: var(--ui-button-height, 36px);
        min-height: var(--ui-button-min-height, 36px);
        border-radius: var(--border-radius);
        color: var(--ui-muted);
    }
    :global(.admin-shell) .admin-control:hover:not(:global(.branding-preview *)) {
        background: var(--ui-hover);
        box-shadow: none;
    }
    :global(.admin-shell)
        .admin-control[data-variant='l1']:hover:not(:global(.branding-preview *)) {
        background: color-mix(in srgb, var(--ui-primary) 90%, var(--ui-text));
    }
    :global(.admin-shell)
        .admin-control[data-variant='l1d']:hover:not(:global(.branding-preview *)) {
        background: color-mix(in srgb, hsl(var(--error)) 90%, var(--ui-text));
    }
    :global(.admin-shell) .admin-control:disabled:not(:global(.branding-preview *)) {
        opacity: 0.4;
        cursor: not-allowed;
        box-shadow: none;
    }
    :global(.admin-shell) .admin-control:focus-visible:not(:global(.branding-preview *)) {
        outline: 2px solid hsl(var(--accent));
        outline-offset: 3px;
    }
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) :global(.icon),
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) :global(.backspace),
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) :global(.iconLeft),
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) :global(.iconRight) {
        display: grid;
        place-items: center;
        margin: 0;
        position: static;
        width: 20px;
        height: 20px;
    }
    :global(.admin-shell) .admin-control:not(:global(.branding-preview *)) :global(.iconLeft) {
        transform: rotate(180deg);
    }
    :global(.admin-shell)
        .admin-control:has(:global(> .children > .admin-select-trigger)):not(
            :global(.branding-preview *)
        ) {
        padding: 0;
        height: auto;
    }
</style>
