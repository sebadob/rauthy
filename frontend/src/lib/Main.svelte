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

    /*!* look dev *!*/
    /*main {*/
    /*    --action: 34 100% 59%;*/
    /*    --accent: 265 100% 53%;*/
    /*    --error: 15 100% 37%;*/

    /*    !* light *!*/
    /*    --text: 200 5% 37%;*/
    /*    --text-high: 200 15% 25%;*/
    /*    --bg: 34 25% 97%;*/
    /*    --bg-high: 34 20% 90%;*/

    /*    !* dark *!*/
    /*    !*--text: 34 5% 75%;*!*/
    /*    !*--text-high: 34 7% 90%;*!*/
    /*    !*--bg: 200 40% 6%;*!*/
    /*    !*--bg-high: 200 20% 17%;*!*/

    /*    color: hsl(var(--text));*/
    /*    background: hsl(var(--bg));*/
    /*}*/
</style>
