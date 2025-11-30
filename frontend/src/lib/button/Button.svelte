<script lang="ts">
    import { fade } from 'svelte/transition';
    import type { Snippet } from 'svelte';
    import Loading from '$lib5/Loading.svelte';

    let {
        type = 'button',
        role = 'button',
        ref = $bindable(),
        id,
        ariaLabel,
        ariaControls,
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
    } = $props();

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
    let showText = $state(!isLoading);
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
    aria-current={ariaCurrent}
    class={cls}
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
        border: 1px solid hsl(var(--action));
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
</style>
