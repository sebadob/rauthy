<script>
    import {spring} from "svelte/motion";

    export let selected = false;
    const selMargin = 11;

    let margin = spring(0, {
        stiffness: 0.15,
        damping: 0.5
    });

    $: margin.set(selected ? selMargin : 0);

    function handleClick() {
        selected = !selected;
    }
</script>

<div
        role="switch"
        aria-checked={selected}
        tabindex="0"
        class="outer"
        class:selectedOuter={selected}
        on:click={handleClick}
        on:keypress={handleClick}
>
    <div class="inner" class:selected style="margin-left: {`${$margin}px`}">
    </div>
</div>

<style>
    .outer {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        padding: 3px;
        height: 20px;
        width: 33px;
        border: 1px solid var(--col-act2a);
        background: var(--col-ghigh);
        border-radius: 10px;
        cursor: pointer;
        box-shadow: inset 0 0 2px 1px var(--col-act2);
    }

    .selectedOuter {
        background: var(--col-ghigh);
    }

    .inner {
        height: 14px;
        width: 14px;
        border-radius: 50%;
        background: var(--col-glow);
    }

    .selected {
        background: var(--col-act2a);
    }
</style>
