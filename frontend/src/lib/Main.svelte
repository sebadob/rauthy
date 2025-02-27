<script lang="ts">
    import {type Snippet} from "svelte";

    let {children}: { children: Snippet } = $props();
</script>

<main>
    {@render children()}
</main>


<style>
    main {
        width: 100%;
        display: flex;
        overflow-y: auto;
    }
</style>