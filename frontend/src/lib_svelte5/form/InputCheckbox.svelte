<script lang="ts">
    import type {Snippet} from 'svelte'

    let {
        checked = $bindable(false),
        disabled,
        ariaLabel = '',
        borderColor = 'hsl(var(--bg-high))',
        name,
        children,
    }: {
        checked: boolean,
        disabled?: boolean,
        ariaLabel: string,
        borderColor?: string,
        name?: string,
        children?: Snippet,
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

<label class="font-label noselect">
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
    <span>
        {@render children?.()}
    </span>
</label>
<!--</div>-->

<style>
    label {
        display: grid;
        grid-template-columns: 1rem auto;
        gap: .66rem;
        cursor: pointer;
    }

    input[type="checkbox"] {
        display: grid;
        place-content: center;
        margin: 0;
        appearance: none;
        width: 1.25rem;
        height: 1.25rem;
        background: hsl(var(--bg));
        border: .15rem solid hsl(var(--bg-high));
        border-radius: .2rem;
        transform: translateY(.1rem);
        cursor: pointer;
    }

    input[type="checkbox"]::before {
        content: "";
        width: 0.8rem;
        height: 0.8rem;
        border-radius: 2px;
        transform: scale(0);
        background: hsl(var(--action));
        transition: 150ms transform ease-in-out;
        transform-origin: bottom left;
        clip-path: polygon(14% 44%, 0 65%, 50% 100%, 100% 16%, 80% 0%, 43% 62%);
    }

    input[type="checkbox"]:checked::before {
        transform: scale(1);
    }

    input[type="checkbox"]:focus-visible {
        outline: 2px solid hsl(var(--accent));
        outline-offset: 2px;
    }

    input[type="checkbox"]:disabled::before {
        background: hsl(var(--text));
    }

    input[type="checkbox"]:disabled {
        cursor: not-allowed;
    }

    label > span {
        transform: translateY(.05rem);
    }
</style>
