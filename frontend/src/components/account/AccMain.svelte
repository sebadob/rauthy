<script lang="ts">
    import {redirectToLogout} from "$utils/helpers.ts";
    import AccInfo from "./AccInfo.svelte";
    import AccEdit from "./AccEdit.svelte";
    import AccMFA from "./AccMFA.svelte";
    import AccPassword from "./AccPassword.svelte";
    import AccWebId from "./AccWebId.svelte";
    import AccDevices from "./AccDevices.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {SessionResponse} from "$api/response/common/session.js";
    import type {UserResponse} from "$api/response/common/user.ts";
    import {TPL_AUTH_PROVIDERS} from "$utils/constants";
    import Template from "$lib5/Template.svelte";
    import type {AuthProvidersTemplate} from "$api/templates/AuthProvider.ts";
    import {onMount} from "svelte";
    import {useParam} from "$state/param.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";

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
    <!--    <div class="wrapperInner">-->
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
                    <AccMFA {session} {user}/>
                {:else if selected === 'WebID'}
                    <AccWebId bind:webIdData/>
                {:else if selected === t.account.devices}
                    <AccDevices bind:session/>
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
                    <AccMFA {session} {user}/>
                {:else if selected === 'WebID'}
                    <AccWebId bind:webIdData/>
                {:else if selected === t.account.devices}
                    <AccDevices bind:session/>
                {/if}
            </div>
        </div>
    {/if}
    <!--    </div>-->
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

    .inner {
        width: min(30rem, 100dvw);
        padding: .5rem;
    }

    .innerPhone {
        width: 100vw;
    }

    .wrapper {
        height: 100dvh;
        max-width: 100dvw;
    }

    /*.wrapperInner {*/
    /*    margin-top: 1rem;*/
    /*    padding: .5rem;*/
    /*    border: 1px solid hsl(var(--bg-high));*/
    /*    border-radius: var(--border-radius);*/
    /*}*/
</style>
