<script lang="ts">
    import { redirectToLogout } from '$utils/helpers';
    import AccInfo from '$lib5/account/AccInfo.svelte';
    import AccEdit from '$lib5/account/AccEdit.svelte';
    import AccMFA from './AccMFA.svelte';
    import AccPassword from './AccPassword.svelte';
    import AccWebId from './AccWebId.svelte';
    import AccDevices from '$lib5/account/AccDevices.svelte';
    import { useI18n } from '$state/i18n.svelte.js';
    import type { UserResponse } from '$api/types/user.ts';
    import { TPL_AUTH_PROVIDERS, TPL_USER_VALUES_CONFIG } from '$utils/constants';
    import Template from '$lib5/Template.svelte';
    import type { AuthProvidersTemplate } from '$api/templates/AuthProvider.ts';
    import { onMount } from 'svelte';
    import { useParam } from '$state/param.svelte';
    import Tabs from '$lib5/tabs/Tabs.svelte';
    import Devices from '$lib5/devices/Devices.svelte';
    import type { WebIdResponse } from '$api/types/web_id.ts';
    import IconLogout from '$icons/IconLogout.svelte';
    import Button from '$lib/button/Button.svelte';
    import AccOther from '$lib/account/AccOther.svelte';
    import AccPAM from '$lib/account/AccPAM.svelte';
    import type { PamUserResponse } from '$api/types/pam';
    import { fetchGet } from '$api/fetch';
    import type { UserValuesConfig } from '$api/templates/UserValuesConfig';

    let {
        user = $bindable(),
        webIdData = $bindable(),
    }: {
        user: UserResponse;
        webIdData: undefined | WebIdResponse;
    } = $props();

    let t = useI18n();

    let innerWidth: undefined | number = $state();
    let config: undefined | UserValuesConfig = $state();
    let providers: AuthProvidersTemplate = $state([]);
    let authProvider = $derived.by(() => {
        if (user.account_type?.startsWith('federated')) {
            return providers.filter(p => p.id === user.auth_provider_id)[0];
        }
    });

    let viewModePhone = $derived(innerWidth && innerWidth < 560);
    let viewModeWideCompact = $derived(innerWidth && innerWidth < 1000);

    let pamUser: undefined | PamUserResponse = $state();

    let selected = $state(t.account.navInfo);
    let tabsWide = $derived.by(() => {
        let tabs = [];
        if (pamUser) {
            tabs.push('PAM');
        }

        tabs = [...tabs, t.account.navMfa, t.account.devices, t.account.navEdit, t.common.password];
        if (!!webIdData) {
            tabs.push('WebID');
        }
        tabs.push(t.account.other);

        return tabs;
    });
    let tabsCompact = $derived([t.account.navInfo, ...tabsWide, t.account.navLogout]);

    onMount(() => {
        fetchPamUser();

        if (useParam('v').get() === 'devices') {
            selected = t.account.devices;
        }
    });

    $effect(() => {
        if (viewModePhone || viewModeWideCompact) {
            selected = t.account.navInfo;
        } else if (pamUser) {
            selected = 'PAM';
        } else {
            selected = t.account.navMfa;
        }
    });

    $effect(() => {
        if (selected === t.account.navLogout) {
            redirectToLogout();
        }
    });

    $effect(() => {
        if (!config) {
            return;
        }

        if (
            (config.birthdate === 'required' && !user.user_values.birthdate) ||
            (config.city === 'required' && !user.user_values.city) ||
            (config.country === 'required' && !user.user_values.country) ||
            (config.given_name === 'required' && !user.given_name) ||
            (config.family_name === 'required' && !user.family_name) ||
            (config.phone === 'required' && !user.user_values.phone) ||
            (config.preferred_username.preferred_username === 'required' &&
                !user.user_values.preferred_username) ||
            (config.street === 'required' && !user.user_values.street) ||
            (config.tz === 'required' && !user.user_values.tz) ||
            (config.zip === 'required' && !user.user_values.zip)
        ) {
            selected = t.account.navEdit;
        }
    });

    async function fetchPamUser() {
        let res = await fetchGet<PamUserResponse>('/auth/v1/pam/users/self');
        if (res.body) {
            pamUser = res.body;
        }
    }
</script>

<svelte:window bind:innerWidth />

<Template id={TPL_AUTH_PROVIDERS} bind:value={providers} />
<Template id={TPL_USER_VALUES_CONFIG} bind:value={config} />

{#snippet header()}
    <h3>{`${user.given_name || ''} ${user.family_name || ''}`}</h3>
{/snippet}

<div class="wrapper">
    {#if viewModePhone}
        <div class="headerPhone">
            {@render header()}
        </div>

        <div class="container">
            <Tabs tabs={tabsCompact} bind:selected />

            <div class="innerPhone">
                {#if selected === t.account.navInfo}
                    <AccInfo
                        bind:user
                        {pamUser}
                        {providers}
                        {authProvider}
                        viewModePhone
                        {webIdData}
                    />
                {:else if selected === 'PAM' && pamUser}
                    <AccPAM bind:pamUser />
                {:else if selected === t.account.navEdit}
                    <AccEdit {config} bind:user viewModePhone />
                {:else if selected === t.common.password}
                    <AccPassword {user} {authProvider} viewModePhone />
                {:else if selected === t.account.navMfa}
                    <AccMFA {user} />
                {:else if selected === 'WebID'}
                    {#if webIdData}
                        <AccWebId bind:webIdData />
                    {/if}
                {:else if selected === t.account.other}
                    <AccOther {user} />
                {:else if selected === t.account.devices}
                    <Devices userId={user.id} />
                {/if}
            </div>
        </div>
    {:else}
        <div class="wide">
            {#if !viewModeWideCompact}
                <div class="info">
                    <AccInfo bind:user {pamUser} {webIdData} {providers} {authProvider} />
                </div>
            {/if}

            <div class="container">
                <Tabs tabs={viewModeWideCompact ? tabsCompact : tabsWide} bind:selected center />

                <div class="inner">
                    {#if selected === t.account.navInfo}
                        <AccInfo bind:user {pamUser} {webIdData} {providers} {authProvider} />
                    {:else if selected === 'PAM' && pamUser}
                        <AccPAM bind:pamUser />
                    {:else if selected === t.account.navEdit}
                        <AccEdit {config} bind:user />
                    {:else if selected === t.common.password}
                        <AccPassword {user} {authProvider} />
                    {:else if selected === t.account.navMfa}
                        <AccMFA {user} />
                    {:else if selected === 'WebID'}
                        {#if webIdData}
                            <AccWebId bind:webIdData />
                        {/if}
                    {:else if selected === t.account.other}
                        <AccOther {user} />
                    {:else if selected === t.account.devices}
                        <AccDevices />
                    {/if}
                </div>
            </div>

            <div class="logout">
                <Button level={-3} onclick={redirectToLogout}>
                    <div title={t.account.navLogout} class="flex gap-05">
                        <IconLogout />
                        {t.account.navLogout}
                    </div>
                </Button>
            </div>
        </div>
    {/if}
</div>

<style>
    .container {
        max-height: calc(100dvh - 5.5rem);
        margin-top: 0.5rem;
        display: flex;
        flex-direction: column;
    }

    .headerPhone {
        margin-left: 0.5rem;
    }

    .info {
        border-right: 1px solid hsl(var(--bg-high));
    }

    .inner,
    .innerPhone {
        padding: 0.5rem;
    }

    .inner {
        margin-top: 0.5rem;
        width: min(33rem, 100dvw);
        max-height: calc(100dvh - 7rem);
        overflow-y: auto;
    }

    .innerPhone {
        width: 100vw;
        max-height: calc(100dvh - 10rem);
        overflow-y: auto;
    }

    .logout {
        position: absolute;
        top: 0.25rem;
        right: 0.5rem;
    }

    .wide {
        height: min(max(50rem, 66dvh), 90dvh);
        padding: 1rem;
        display: flex;
        gap: 1rem;
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / 0.25);
    }

    .wide:has(.info) {
        border-radius: 4.5rem var(--border-radius) var(--border-radius) var(--border-radius);
    }

    .wide .inner {
        max-height: 100%;
        overflow-y: auto;
    }

    .wrapper {
        height: 100dvh;
        max-width: 100dvw;
        display: flex;
        flex-direction: column;
    }

    .wrapper:has(.wide) {
        flex-direction: row;
        align-items: center;
    }
</style>
