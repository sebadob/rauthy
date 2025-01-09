<script lang="ts">
    import {type Snippet} from "svelte";

    let {children}: { children: Snippet } = $props();
</script>

<div>
    {@render children()}
</div>


<style>
    div {
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>