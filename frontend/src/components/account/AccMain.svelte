<script>
    import {redirectToLogout} from "../../utils/helpers.js";
    import AccInfo from "./AccInfo.svelte";
    import AccNav from "./AccNav.svelte";
    import AccEdit from "./AccEdit.svelte";
    import {tweened} from "svelte/motion";
    import AccMFA from "./AccMFA.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import AccPassword from "./AccPassword.svelte";

    export let t;

    export let sessionInfo = {};
    export let user = {};

    let innerWidth;
    $: viewModePhone = innerWidth < 500;

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

<svelte:window bind:innerWidth/>

{#if viewModePhone}
    <div class="wrapper">
        <LangSelector absolute absoluteRight updateBackend/>

        <div class="headerPhone">
            <h2>{t.user} {t.account}</h2>
        </div>

        <div class="containerPhone">
            <AccNav bind:t bind:selected showWide />

            <div class="innerPhone">
                <div style="opacity: {$op}">
                    {#if content === t.navInfo}
                        <AccInfo bind:t bind:user viewModePhone />
                    {:else if content === t.navEdit}
                        <AccEdit bind:t bind:user viewModePhone />
                    {:else if content === t.navPassword}
                        <AccPassword bind:t bind:user viewModePhone />
                    {:else if content === t.navMfa}
                        <AccMFA bind:t bind:sessionInfo bind:user/>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{:else}
    <LangSelector absolute updateBackend/>

    <div class="header">
        <h2>{t.user} {t.account}</h2>
    </div>

    <div class="container">
        <AccNav bind:t bind:selected/>

        <div class="inner borderLeft">
            <div style="opacity: {$op}">
                {#if content === t.navInfo}
                    <AccInfo bind:t bind:user />
                {:else if content === t.navEdit}
                    <AccEdit bind:t bind:user />
                {:else if content === t.navPassword}
                    <AccPassword bind:t bind:user />
                {:else if content === t.navMfa}
                    <AccMFA bind:t bind:sessionInfo bind:user/>
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
    .borderLeft {
        padding-left: 20px;
        border-left: 1px solid var(--col-acnt);
    }

    .container {
        display: flex;
    }

    .containerPhone {
        display: flex;
        flex-direction: column;
    }

    .header {
        width: 100%;
        margin-left: 150px;
        margin-bottom: 30px;
    }

    .headerPhone {
        margin-left: 10px;
    }

    .inner {
        width: 30rem;
    }

    .innerPhone {
        width: 100vw;
    }

    .wrapper {
        height: 100dvh;
    }
</style>
