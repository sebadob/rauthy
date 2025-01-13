<script>
    import { run } from 'svelte/legacy';

    import {getAuthProvidersTemplate, redirectToLogout} from "../../utils/helpers.js";
    import AccInfo from "./AccInfo.svelte";
    import AccNav from "./AccNav.svelte";
    import AccEdit from "./AccEdit.svelte";
    import {tweened} from "svelte/motion";
    import AccMFA from "./AccMFA.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import AccPassword from "./AccPassword.svelte";
    import AccWebId from "./AccWebId.svelte";
    import {onMount} from "svelte";
    import AccDevices from "./AccDevices.svelte";


    
    /**
     * @typedef {Object} Props
     * @property {any} t
     * @property {any} [sessionInfo]
     * @property {any} [user]
     * @property {any} webIdData - webIdData will stay undefined if it is not enabled in the backend
     */

    /** @type {Props} */
    let {
        t,
        sessionInfo,
        user = $bindable({}),
        webIdData = $bindable()
    } = $props();

    let innerWidth = $state();
    let providers = $state();
    let authProvider = $state();


    let op = tweened(1.0, {
        duration: 100,
    })

    let content = $state(t.navInfo);
    let selected = $state(t.navInfo);

    onMount(async () => {
        providers = await getAuthProvidersTemplate();
    });

    function animate() {
        op.set(0)
            .then(() => content = selected)
            .then(() => op.set(1.0));
    }
    let viewModePhone = $derived(innerWidth < 500);
    run(() => {
        if (selected) {
            animate();
        }
    });
    run(() => {
        if (selected === t.navLogout) {
            redirectToLogout();
        }
    });
    run(() => {
        if (providers) {
            if (user.account_type?.startsWith('federated')) {
                authProvider = providers.filter(p => p.id === user.auth_provider_id)[0];
            }
        }
    });
</script>

<svelte:window bind:innerWidth/>

{#if viewModePhone}
    <div class="wrapper">
        <LangSelector absolute absoluteRight updateBackend/>

        <div class="headerPhone">
            <h3>{t.account}</h3>
        </div>

        <div class="container">
            <AccNav {t} bind:selected />

            <div class="innerPhone">
                <div style="opacity: {$op}">
                    {#if content === t.navInfo}
                        <AccInfo {t} bind:user {webIdData} viewModePhone {authProvider}/>
                    {:else if content === t.navEdit}
                        <AccEdit {t} bind:user viewModePhone/>
                    {:else if content === t.navPassword}
                        <AccPassword {t} {user} {authProvider} viewModePhone/>
                    {:else if content === t.navMfa}
                        <AccMFA {t} {sessionInfo} {user}/>
                    {:else if content === 'WebID'}
                        <AccWebId {t} bind:webIdData />
                    {:else if content === t.devices}
                        <AccDevices {t} bind:sessionInfo/>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{:else}
    <div class="wrapper">
        <LangSelector absolute updateBackend/>

        <div class="header">
            <h3>{t.account}</h3>
        </div>

        <div class="container">
            <AccNav {t} bind:selected showWebId={!!webIdData} />

            <div class="inner">
                <div style="opacity: {$op}">
                    {#if content === t.navInfo}
                        <AccInfo {t} bind:user {webIdData} {authProvider}/>
                    {:else if content === t.navEdit}
                        <AccEdit {t} bind:user/>
                    {:else if content === t.navPassword}
                        <AccPassword {t} {user} {authProvider}/>
                    {:else if content === t.navMfa}
                        <AccMFA {t} {sessionInfo} {user}/>
                    {:else if content === 'WebID'}
                        <AccWebId {t} bind:webIdData/>
                    {:else if content === t.devices}
                        <AccDevices {t} bind:sessionInfo/>
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
        height: calc(100dvh - 20px);
    }
</style>
