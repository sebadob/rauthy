<script>
    import {getAuthProvidersTemplate, redirectToLogout} from "../../utils/helpers.js";
    import AccInfo from "./AccInfo.svelte";
    import AccNav from "./AccNav.svelte";
    import AccEdit from "./AccEdit.svelte";
    import {tweened} from "svelte/motion";
    import AccMFA from "./AccMFA.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import AccPassword from "./AccPassword.svelte";
    import AccWebId from "./AccWebId.svelte";
    import {onMount} from "svelte";
    import AccDevices from "./AccDevices.svelte";

    export let t;

    export let sessionInfo = {};
    export let user = {};
    // webIdData will stay undefined if it is not enabled in the backend
    export let webIdData;

    let innerWidth;
    let providers;
    let authProvider;

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

    $: if (providers) {
        if (user.account_type.startsWith('federated')) {
            authProvider = providers.filter(p => p.id === user.auth_provider_id)[0];
        }
    }

    onMount(async () => {
        providers = await getAuthProvidersTemplate();
    });

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
            <h3>{t.user} {t.account}</h3>
        </div>

        <div class="container">
            <AccNav bind:t bind:selected showWide/>

            <div class="innerPhone">
                <div style="opacity: {$op}">
                    {#if content === t.navInfo}
                        <AccInfo bind:t bind:user bind:webIdData viewModePhone bind:authProvider/>
                    {:else if content === t.navEdit}
                        <AccEdit bind:t bind:user viewModePhone/>
                    {:else if content === t.navPassword}
                        <AccPassword bind:t bind:user bind:authProvider viewModePhone/>
                    {:else if content === t.navMfa}
                        <AccMFA bind:t bind:sessionInfo bind:user/>
                    {:else if content === 'WebID'}
                        <AccWebId bind:t bind:webIdData viewModePhone/>
                    {:else if content === t.devices}
                        <AccDevices bind:t bind:sessionInfo/>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{:else}
    <div class="wrapper">
        <LangSelector absolute updateBackend/>

        <div class="header">
            <h3>{t.user} {t.account}</h3>
        </div>

        <div class="container">
            <AccNav bind:t bind:selected showWebId={!!webIdData} showWide/>

            <div class="inner">
                <div style="opacity: {$op}">
                    {#if content === t.navInfo}
                        <AccInfo bind:t bind:user bind:webIdData bind:authProvider/>
                    {:else if content === t.navEdit}
                        <AccEdit bind:t bind:user/>
                    {:else if content === t.navPassword}
                        <AccPassword bind:t bind:user bind:authProvider/>
                    {:else if content === t.navMfa}
                        <AccMFA bind:t bind:sessionInfo bind:user/>
                    {:else if content === 'WebID'}
                        <AccWebId bind:t bind:webIdData/>
                    {:else if content === t.devices}
                        <AccDevices bind:t bind:sessionInfo/>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .container {
        display: flex;
        flex-direction: column;
    }

    .header {
        width: 100%;
        margin-left: 10px;
        margin-bottom: -10px;
    }

    .headerPhone {
        margin-left: 10px;
        margin-bottom: -10px;
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
