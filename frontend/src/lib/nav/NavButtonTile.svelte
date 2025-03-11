<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import type {Snippet} from "svelte";

    let {
        selected,
        pictureLeft,
        onclick,
        children,
    }: {
        selected: boolean,
        pictureLeft?: boolean,
        onclick: () => void,
        children: Snippet,
    } = $props();

    let ref: undefined | HTMLButtonElement = $state();
</script>

<li class:pictureLeft aria-current={selected ? 'page' : 'false'}>
    <div>
        <Button bind:ref invisible {onclick}>
            <div>
                {@render children()}
            </div>
        </Button>
    </div>
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
        color: hsl(var(--text));
        text-wrap: wrap;
        transition: all 150ms;
    }

    li:nth-of-type(2n + 1) {
        background: hsla(var(--bg-high) / .15);
    }

    li[aria-current="page"] {
        border-right: 1px solid hsl(var(--accent));
        background: hsla(var(--bg-high) / .8);
    }

    li[aria-current="page"] div {
        color: hsl(var(--text-high));
    }

    li:hover {
        background: hsl(var(--bg-high));
        border-right: 1px solid hsl(var(--action));
    }

    li:hover div {
        color: hsl(var(--action));
    }

    .pictureLeft {
        margin: .1rem .25rem;
        padding: 0;
        border-radius: .65rem var(--border-radius) var(--border-radius) .65rem;
    }

    .pictureLeft > div {
        height: 2.25rem;
        overflow: clip;
        margin-left: -.1rem;
    }
</style>
