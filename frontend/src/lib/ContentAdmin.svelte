<script lang="ts">
    import type { Snippet } from 'svelte';

    let {
        children,
    }: {
        children: Snippet;
    } = $props();
</script>

<div class="content">
    {@render children()}
</div>

<style>
    .content {
        flex: 1;
        padding: 2.2rem 1rem 0 1rem;
        overflow: auto;
    }

    @media (max-width: 800px) {
        .content {
            padding: 1.35rem 0.25rem 0 0.25rem;
        }
    }
</style>
