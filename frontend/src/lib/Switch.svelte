<script lang="ts">
    import type { Snippet } from 'svelte';

    let {
        checked = $bindable(false),
        disabled,
        ariaLabel = '',
        name = '',
        labelWidth,
        children,
    }: {
        checked: boolean;
        disabled?: boolean;
        ariaLabel: string;
        name?: string;
        labelWidth?: string;
        children: Snippet;
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

<div class="flex">
    <div class="label font-label noselect" style:width={labelWidth}>
        {@render children()}
    </div>
    <label class="switch">
        <input
            type="checkbox"
            {name}
            bind:checked
            {disabled}
            aria-checked={checked}
            aria-label={ariaLabel}
            {onclick}
            {onkeydown}
        />
        <span class="slider slider-round"></span>
    </label>
</div>

<style>
    .label {
        margin: 0 0.5rem -0.2rem 0;
        word-break: break-word;
    }

    .switch {
        position: relative;
        width: 2rem;
        height: 1.14rem;
    }

    /* outer slider */
    .slider {
        position: absolute;
        cursor: pointer;
        top: -0.02rem;
        left: 0;
        right: 0;
        bottom: -0.02rem;
        background-color: hsl(var(--bg-high));
        -webkit-transition: 333ms;
        transition: 333ms;
    }

    input[type='checkbox']:disabled + .slider {
        cursor: not-allowed;
    }

    /* inner slider knob */
    .slider:before {
        position: absolute;
        content: '';
        height: 1rem;
        aspect-ratio: 1;
        left: 0.15rem;
        bottom: 0.04rem;
        background-color: hsl(var(--bg));
        -webkit-transition: 150ms;
        transition: 150ms;
    }

    input[type='checkbox'] {
        appearance: none;
    }

    input[aria-checked='true'] + .slider {
        background-color: hsl(var(--action));
    }

    input[aria-checked='true']:disabled + .slider {
        background-color: hsl(var(--bg-high));
    }

    input:focus-visible + .slider {
        outline: 2px solid hsl(var(--accent));
        outline-offset: 2px;
    }

    input[aria-checked='true'] + .slider:before {
        --webkit-transform: translateX(0.78rem);
        -ms-transform: translateX(0.78rem);
        transform: translateX(0.78rem);
    }

    .slider.slider-round {
        border-radius: 1.1rem;
    }

    .slider.slider-round:before {
        border-radius: 50%;
    }
</style>
