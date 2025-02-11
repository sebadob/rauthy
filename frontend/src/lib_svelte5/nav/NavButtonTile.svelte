<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import type {Snippet} from "svelte";

    let {
        selected,
        onclick,
        children,
    }: {
        selected: boolean,
        onclick: () => void,
        children: Snippet,
    } = $props();
</script>

<li aria-current={selected ? 'page' : 'false'}>
    <Button invisible {onclick}>
        <div>
            {@render children()}
        </div>
    </Button>
</li>

<style>
    li {
        border-left: 2px solid transparent;
        border-radius: var(--border-radius);
        margin: 0;
        padding: .25rem .5rem;
        list-style: none;
        transition: all 150ms;
    }

    li div {
        text-wrap: wrap;
    }

    li:nth-of-type(2n + 1) {
        background: hsla(var(--bg-high) / .15);
    }

    li[aria-current="page"] {
        border-color: hsl(var(--accent));
    }

    li[aria-current="page"] div {
        color: hsl(var(--text-high));
    }

    li:hover {
        border-color: hsl(var(--action));
    }
</style>
