<script>
    import {redirectToLogout} from "../../utils/helpers.js";
    import AccInfo from "./AccInfo.svelte";
    import AccNav from "./AccNav.svelte";
    import AccEdit from "./AccEdit.svelte";
    import {tweened} from "svelte/motion";
    import AccMFA from "./AccMFA.svelte";

    export let sessionInfo = {};
    export let user = {};

    let op = tweened(1.0, {
        duration: 250,
    })

    let content = 'INFO';
    let selected = 'INFO';

    $: if (selected) {
        animate();
    }

    $: if (selected === 'LOGOUT') {
        redirectToLogout();
    }

    function animate() {
        op.set(0)
            .then(() => content = selected)
            .then(() => op.set(1.0));
    }
</script>

<div class="header">
    <h2>User Account</h2>
</div>

<div class="container">
    <AccNav bind:selected/>

    <div class="inner">
        <div style="opacity: {$op}">
            {#if content === 'INFO'}
                <AccInfo bind:user/>
            {:else if content === 'EDIT'}
                <AccEdit bind:user/>
            {:else if content === 'MFA'}
                <AccMFA bind:sessionInfo bind:user/>
            {/if}
        </div>
    </div>
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
