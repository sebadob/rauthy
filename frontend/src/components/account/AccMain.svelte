<script>
    import {redirectToLogout} from "../../utils/helpers.js";
    import AccInfo from "./AccInfo.svelte";
    import AccNav from "./AccNav.svelte";
    import AccEdit from "./AccEdit.svelte";
    import {tweened} from "svelte/motion";
    import AccMFA from "./AccMFA.svelte";
    import LangSelector from "$lib/LangSelector.svelte";

    export let t;

    export let sessionInfo = {};
    export let user = {};

    let op = tweened(1.0, {
        duration: 100,
    })

    let content = t.info;
    let selected = t.info;

    $: if (selected) {
        animate();
    }

    $: if (selected === t.navLogout) {
        redirectToLogout();
    }

    function animate() {
        op.set(0)
            .then(() => content = selected)
            .then(() => op.set(1.0));
    }
</script>

<div class="header">
    <h2>{t.user} {t.account}</h2>
</div>

<div class="container">
    <AccNav bind:t bind:selected/>

    <div class="inner">
        <div style="opacity: {$op}">
            {#if content === t.navInfo}
                <AccInfo bind:t bind:user/>
            {:else if content === t.navEdit}
                <AccEdit bind:t bind:user/>
            {:else if content === t.navMfa}
                <AccMFA bind:t bind:sessionInfo bind:user/>
            {/if}
        </div>
    </div>

    <LangSelector absolute />
</div>

<style>
    .container {
        display: flex;
        min-height: 320px;
    }

    .header {
        width: 100%;
        margin-left: 150px;
        margin-bottom: 30px;
    }

    .inner {
        width: 380px;
        padding-left: 20px;
        border-left: 1px solid var(--col-acnt);
    }
</style>
