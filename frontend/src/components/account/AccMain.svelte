<script lang="ts">
    import { run } from 'svelte/legacy';
    import {redirectToLogout} from "../../utils/helpers";
    import AccInfo from "./AccInfo.svelte";
    import AccNav from "./AccNav.svelte";
    import AccEdit from "./AccEdit.svelte";
    import {tweened} from "svelte/motion";
    import AccMFA from "./AccMFA.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import AccPassword from "./AccPassword.svelte";
    import AccWebId from "./AccWebId.svelte";
    import AccDevices from "./AccDevices.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {SessionResponse} from "$api/response/common/session.js";
    import type {UserResponse} from "$api/response/common/user.ts";
    import {TPL_AUTH_PROVIDERS} from "../../utils/constants";
    import Template from "$lib5/Template.svelte";
    import type {AuthProvidersTemplate} from "$api/templates/AuthProvider.ts";

    let {
        session: session,
        user = $bindable(),
        webIdData = $bindable()
    }: {
        session: SessionResponse,
        user: UserResponse,
        webIdData: any,
    } = $props();

    let t = useI18n();

    let innerWidth: undefined | number = $state();
    let providers: AuthProvidersTemplate = $state([]);
    let authProvider = $derived.by(() => {
        if (user.account_type?.startsWith('federated')) {
            return providers.filter(p => p.id === user.auth_provider_id)[0];
        }
    });

    let op = tweened(1.0, {
        duration: 100,
    })

    let content = $state(t.account.navInfo);
    let selected = $state(t.account.navInfo);

    function animate() {
        op.set(0)
            .then(() => content = selected)
            .then(() => op.set(1.0));
    }
    let viewModePhone = $derived(innerWidth && innerWidth < 500);
    run(() => {
        if (selected) {
            animate();
        }
    });

    $effect(() => {
        if (selected === t.account.navLogout) {
            redirectToLogout();
        }
    });
</script>

<svelte:window bind:innerWidth/>

<Template id={TPL_AUTH_PROVIDERS} bind:value={providers}/>

{#if viewModePhone}
    <div class="wrapper">
        <LangSelector absolute absoluteRight updateBackend/>

        <div class="headerPhone">
            <h3>{t.account.account}</h3>
        </div>

        <div class="container">
            <AccNav bind:selected />

            <div class="innerPhone">
                <div style="opacity: {$op}">
                    {#if content === t.account.navInfo}
                        <AccInfo bind:user {webIdData} viewModePhone {providers} {authProvider}/>
                    {:else if content === t.account.navEdit}
                        <AccEdit bind:user viewModePhone/>
                    {:else if content === t.common.password}
                        <AccPassword {user} {authProvider} viewModePhone/>
                    {:else if content === t.account.navMfa}
                        <AccMFA {session} {user}/>
                    {:else if content === 'WebID'}
                        <AccWebId bind:webIdData />
                    {:else if content === t.account.devices}
                        <AccDevices bind:session/>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{:else}
    <div class="wrapper">
        <LangSelector absolute updateBackend/>

        <div class="header">
            <h3>{t.account.account}</h3>
        </div>

        <div class="container">
            <AccNav bind:selected showWebId={!!webIdData} />

            <div class="inner">
                <div style="opacity: {$op}">
                    {#if content === t.account.navInfo}
                        <AccInfo bind:user {webIdData} {providers} {authProvider}/>
                    {:else if content === t.account.navEdit}
                        <AccEdit bind:user/>
                    {:else if content === t.common.password}
                        <AccPassword {user} {authProvider}/>
                    {:else if content === t.account.navMfa}
                        <AccMFA {session} {user}/>
                    {:else if content === 'WebID'}
                        <AccWebId bind:webIdData/>
                    {:else if content === t.account.devices}
                        <AccDevices bind:session/>
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
