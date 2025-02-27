<script lang="ts">
    import {redirectToLogout} from "$utils/helpers.ts";
    import AccInfo from "$lib5/account/AccInfo.svelte";
    import AccEdit from "$lib5/account/AccEdit.svelte";
    import AccMFA from "./AccMFA.svelte";
    import AccPassword from "./AccPassword.svelte";
    import AccWebId from "./AccWebId.svelte";
    import AccDevices from "$lib5/account/AccDevices.svelte";
    import {useI18n} from "$state/i18n.svelte.js";
    import type {UserResponse} from "$api/types/user.ts";
    import {TPL_AUTH_PROVIDERS} from "$utils/constants.ts";
    import Template from "$lib5/Template.svelte";
    import type {AuthProvidersTemplate} from "$api/templates/AuthProvider.ts";
    import {onMount} from "svelte";
    import {useParam} from "$state/param.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import Devices from "$lib5/devices/Devices.svelte";
    import type {WebIdResponse} from "$api/types/web_id.ts";

    let {
        user = $bindable(),
        webIdData = $bindable()
    }: {
        user: UserResponse,
        webIdData: WebIdResponse,
    } = $props();

    let t = useI18n();

    let innerWidth: undefined | number = $state();
    let providers: AuthProvidersTemplate = $state([]);
    let authProvider = $derived.by(() => {
        if (user.account_type?.startsWith('federated')) {
            return providers.filter(p => p.id === user.auth_provider_id)[0];
        }
    });

    let viewModePhone = $derived(innerWidth && innerWidth < 500);

    let selected = $state(t.account.navInfo);
    let tabs = $state(!!webIdData ?
        [
            t.account.navInfo,
            t.account.navEdit,
            t.common.password,
            t.account.navMfa, 'WebID',
            t.account.devices,
            t.account.navLogout,
        ]
        : [
            t.account.navInfo,
            t.account.navEdit,
            t.common.password,
            t.account.navMfa,
            t.account.devices,
            t.account.navLogout,
        ]);

    onMount(() => {
        if (useParam('v').get() === 'devices') {
            selected = t.account.devices;
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

{#snippet header()}
    <h3>{`${user.given_name} ${user.family_name || ''}`}</h3>
{/snippet}

<div class="wrapper">
    {#if viewModePhone}
        <div class="headerPhone">
            {@render header()}
        </div>

        <div class="container">
            <Tabs {tabs} bind:selected/>

            <div class="innerPhone">
                {#if selected === t.account.navInfo}
                    <AccInfo bind:user {providers} {authProvider} viewModePhone {webIdData}/>
                {:else if selected === t.account.navEdit}
                    <AccEdit bind:user viewModePhone/>
                {:else if selected === t.common.password}
                    <AccPassword {user} {authProvider} viewModePhone/>
                {:else if selected === t.account.navMfa}
                    <AccMFA {user}/>
                {:else if selected === 'WebID'}
                    <AccWebId bind:webIdData/>
                {:else if selected === t.account.devices}
                    <Devices viewMode="account"/>
                {/if}
            </div>
        </div>
    {:else}
        <div class="header">
            {@render header()}
        </div>

        <div class="container">
            <Tabs {tabs} bind:selected center/>

            <div class="inner">
                {#if selected === t.account.navInfo}
                    <AccInfo bind:user {webIdData} {providers} {authProvider}/>
                {:else if selected === t.account.navEdit}
                    <AccEdit bind:user/>
                {:else if selected === t.common.password}
                    <AccPassword {user} {authProvider}/>
                {:else if selected === t.account.navMfa}
                    <AccMFA {user}/>
                {:else if selected === 'WebID'}
                    <AccWebId bind:webIdData/>
                {:else if selected === t.account.devices}
                    <AccDevices/>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .container {
        margin-top: .5rem;
        display: flex;
        flex-direction: column;
    }

    .header {
        /*width: 100%;*/
        margin-left: .25rem;
    }

    .headerPhone {
        margin-left: .5rem;
    }

    .inner, .innerPhone {
        padding: .5rem;
    }

    .inner {
        margin-top: .5rem;
        width: min(32rem, 100dvw);
        max-height: calc(100dvh - 7rem);
        overflow-y: auto;
    }

    .innerPhone {
        width: 100vw;
        max-height: calc(100dvh - 8.5rem);
        overflow-y: auto;
    }

    .wrapper {
        height: 100dvh;
        max-width: 100dvw;
    }
</style>
