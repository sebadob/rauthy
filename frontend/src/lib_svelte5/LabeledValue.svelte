<script lang="ts">
    import type {Snippet} from "svelte";

    let {
        label,
        title,
        mono,
        button,
        children,
    }: {
        label?: string,
        title?: string,
        mono?: boolean,
        button?: Snippet,
        children: Snippet,
    } = $props();
</script>

<div class="container">
    {#if label}
        <div class="label font-label">
            {label}
        </div>
    {/if}
    <div class="flex gap-05">
        <div title={title || label || ''} class={mono ? 'font-mono' : ''}>
            {@render children()}
        </div>
        {#if button}
            <div class="button">
                {@render button()}
            </div>
        {/if}
    </div>
</div>

<style>
    .button {
        margin-top: -.15rem;
        margin-bottom: -.5rem;
    }

    .container {
        margin: .25rem 0;
    }

    .label {
        margin-bottom: -.45rem;
        font-size: .85rem;
        color: hsla(var(--text) / .7);
    }
</style>