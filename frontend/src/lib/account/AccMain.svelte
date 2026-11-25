<script lang="ts">
    import { normalizeClientUri } from '#utils/helpers.ts';
    import AccInfo from '#lib/account/AccInfo.svelte';
    import AccEdit from '#lib/account/AccEdit.svelte';
    import AccMFA from './AccMFA.svelte';
    import AccPassword from './AccPassword.svelte';
    import AccWebId from './AccWebId.svelte';
    import { useI18n } from '#state/i18n.svelte.js';
    import type { UserResponse } from '#api/types/user.ts';
    import { TPL_AUTH_PROVIDERS, TPL_USER_VALUES_CONFIG } from '#utils/constants.ts';
    import Template from '#lib/Template.svelte';
    import type { AuthProvidersTemplate } from '#api/templates/AuthProvider.ts';
    import { onMount, setContext } from 'svelte';
    import { useParam } from '#state/param.svelte.ts';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import Devices from '#lib/devices/Devices.svelte';
    import type { WebIdResponse } from '#api/types/web_id.ts';
    import Button from '#lib/button/Button.svelte';
    import ThemeSwitch from '#lib/ThemeSwitch.svelte';
    import LangSelector from '#lib/LangSelector.svelte';
    import { useSession } from '#state/session.svelte.ts';
    import AccOther from '#lib/account/AccOther.svelte';
    import AccPAM from '#lib/account/AccPAM.svelte';
    import AccPrivacy from '#lib/account/AccPrivacy.svelte';
    import type { PamUserResponse } from '#api/types/pam.ts';
    import { fetchGet, fetchPost } from '#api/fetch.ts';
    import type { UserValuesConfig } from '#api/templates/UserValuesConfig.ts';
    import IconHome from '#icons/IconHome.svelte';
    import IconArrowLeft from '#icons/IconArrowLeft.svelte';
    import UserPicture from '#lib/UserPicture.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import Section from '#lib/Section.svelte';
    import AccountActions from './AccountActions.svelte';

    setContext('rauthy-ui', true);

    let {
        user = $bindable(),
        webIdData = $bindable(),
    }: {
        user: UserResponse;
        webIdData: undefined | WebIdResponse;
    } = $props();

    let t = useI18n();
    let session = useSession('account');

    let config: undefined | UserValuesConfig = $state();
    let providers: AuthProvidersTemplate = $state([]);
    let authProvider = $derived.by(() => {
        if (user.account_type?.startsWith('federated')) {
            return providers.filter(p => p.id === user.auth_provider_id)[0];
        }
    });

    // a user holding any `rauthy_admin*` role (full or delegated group admin) gets a
    // link to the Admin UI from here. This is how a group admin self-manages: it cannot manage
    // itself inside the Admin UI (it's an admin there), so it uses this dashboard instead.
    let showAdminLink = $derived(session.isAnyAdmin());

    let pamUser: undefined | PamUserResponse = $state();

    let redirectUri = useParam('redirect_uri');
    const view = useParam('v');
    let redirectUriValidated = $state('');

    let selected = $state(t.account.navInfo);
    const tabs = $derived([
        t.account.navInfo,
        t.account.navEdit,
        t.common.password,
        t.account.navMfa,
        t.account.devices,
        ...(pamUser ? ['PAM'] : []),
        ...(webIdData ? ['WebID'] : []),
        t.account.privacy.privacy,
        t.account.attributes ?? t.account.other,
    ]);

    const title = $derived(
        [user.given_name, user.family_name].filter(Boolean).join(' ') || user.email,
    );
    const initials = $derived(
        (user.given_name?.[0] || user.email[0]) + (user.family_name?.[0] || ''),
    );

    onMount(() => {
        fetchPamUser();
        if (view.get() === 'devices') selected = t.account.devices;
    });

    $effect(() => {
        if (!config || view.get() === 'devices' || view.get() === 'info') {
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

    $effect(() => {
        let uri = redirectUri.get();
        if (uri) {
            fetchRedirectClientInfo(uri);
        }
    });

    async function fetchRedirectClientInfo(redirectUri: string) {
        let res = await fetchPost('/auth/v1/clients/validate_uri', {
            redirect_uri: redirectUri,
        });
        if (res.status === 200) {
            redirectUriValidated = normalizeClientUri(redirectUri);
        } else {
            console.error('Invalid `redirect_uri`', redirectUri);
        }
    }

    async function fetchPamUser() {
        let res = await fetchGet<PamUserResponse>('/auth/v1/pam/users/self');
        if (res.body) {
            pamUser = res.body;
        }
    }
</script>

<Template id={TPL_AUTH_PROVIDERS} bind:value={providers} />
<Template id={TPL_USER_VALUES_CONFIG} bind:value={config} />

<div class="account-shell ui-shell ui-canvas">
    <div class="account">
        <h1>{t.account.account}</h1>
        <EntityHeader {title} subtitle={user.email}>
            {#snippet actions()}
                {#if redirectUriValidated}
                    <Button
                        ariaLabel={t.account.account}
                        level={2}
                        onclick={() => (window.location.href = redirectUriValidated)}
                    >
                        <IconArrowLeft width="1rem" />
                        <IconHome width="1rem" />
                    </Button>
                {/if}
            {/snippet}
            <UserPicture
                fallbackCharacters={initials}
                userId={user.id}
                bind:pictureId={user.picture_id}
                size="large"
            />
        </EntityHeader>

        <Tabs {tabs} bind:selected />

        {#if selected === t.account.navMfa}
            <AccMFA bind:user />
        {:else if selected === t.common.password}
            <AccPassword {user} {authProvider} />
        {:else if selected === 'WebID' && webIdData}
            <AccWebId bind:webIdData />
        {:else if selected === t.account.privacy.privacy}
            <AccPrivacy {user} />
        {:else}
            <Section>
                {#if selected === t.account.navInfo}
                    <AccInfo bind:user {pamUser} {providers} {authProvider} {webIdData} />
                {:else if selected === t.account.navEdit}
                    <AccEdit {config} bind:user />
                {:else if selected === t.account.devices}
                    <Devices userId={user.id} />
                {:else if selected === 'PAM' && pamUser}
                    <AccPAM bind:pamUser />
                {:else if selected === (t.account.attributes ?? t.account.other)}
                    <AccOther {user} />
                {/if}
            </Section>
        {/if}

        <footer>
            <div class="preferences">
                <ThemeSwitch /><LangSelector openTop borderless uppercase />
            </div>
            <AccountActions {showAdminLink} />
        </footer>
    </div>
</div>

<style>
    .account-shell {
        width: 100%;
        min-width: 0;
        min-height: 100%;
        align-self: flex-start;
        color: var(--ui-text);
        padding: clamp(1rem, 4vw, 3rem);
    }

    .account {
        width: min(100%, 64rem);
        margin-inline: auto;
    }

    h1 {
        margin: 0 0 var(--ui-space);
        font-family: var(--font-mono);
        font-size: 0.75rem;
        font-weight: 500;
        letter-spacing: 0.16em;
        text-transform: uppercase;
        color: var(--ui-muted);
    }

    footer,
    .preferences {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    footer {
        justify-content: space-between;
        margin-top: var(--ui-space);
        padding-block: 0.5rem;
        border-top: 1px dashed var(--ui-outline);
    }
</style>
