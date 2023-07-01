<script>
    import {fade} from "svelte/transition";

    const tDur = 100;
</script>

<div
        class="content"
        in:fade={{delay: tDur * 2, duration: tDur}}
        out:fade={{duration: tDur}}
>
    <slot></slot>
</div>

<style>
    .content {
        height: 100vh;
        display: flex;
        flex-direction: column;
        flex: 1;
        justify-content: center;
        align-items: center;
        overflow-y: auto;
    }
</style>
