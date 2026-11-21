<script lang="ts">
    import CheckIcon from '#lib/CheckIcon.svelte';
    import { buildWebIdUri, formatDateFromTs, saveProviderToken } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';
    import { PKCE_VERIFIER_UPSTREAM, TPL_IS_OTP_ENABLED } from '#utils/constants.ts';
    import { useI18n } from '#state/i18n.svelte.js';
    import type { UserResponse } from '#api/types/user.ts';
    import type {
        AuthProvidersTemplate,
        AuthProviderTemplate,
    } from '#api/templates/AuthProvider.ts';
    import type { WebIdResponse } from '#api/types/web_id.ts';
    import type { ProviderLoginRequest } from '#api/types/auth_provider.ts';
    import { fetchDelete, fetchGet, fetchPost } from '#api/fetch.ts';
    import ButtonAuthProvider from '../ButtonAuthProvider.svelte';
    import { fetchSolvePow } from '#utils/pow.ts';
    import { generatePKCE } from '#utils/pkce.ts';
    import type { PamUserResponse } from '#api/types/pam.ts';
    import Template from '#lib/Template.svelte';
    import type { OtpResponse } from '#api/types/otp.ts';

    let {
        user = $bindable(),
        pamUser,
        providers,
        authProvider,
        webIdData,
    }: {
        user: UserResponse;
        pamUser: undefined | PamUserResponse;
        providers: AuthProvidersTemplate;
        authProvider: undefined | AuthProviderTemplate;
        webIdData: undefined | WebIdResponse;
    } = $props();

    let t = useI18n();

    let unlinkErr = $state(false);
    let showModal = $state(false);

    let isFederated = $derived(user.account_type?.startsWith('federated'));
    let accType = $derived(
        isFederated ? `${user.account_type}: ${authProvider?.name || ''}` : user.account_type,
    );

    let isLoading = $state(false);

    let isOtpEnabled = $state(false);
    let hasOtp = $state(false);

    $effect(() => {
        if (isOtpEnabled) {
            fetchOtps();
        }
    });

    async function fetchOtps() {
        let res = await fetchGet<OtpResponse[]>(`/auth/v1/users/${user.id}/otp`);
        if (res.body) {
            for (let otp of res.body) {
                if (otp.is_active) {
                    hasOtp = true;
                    return;
                }
            }
        }
    }

    function linkProvider(id: string) {
        generatePKCE().then(pkce => {
            if (pkce) {
                localStorage.setItem(PKCE_VERIFIER_UPSTREAM, pkce.verifier);
                providerLoginPkce(id, pkce.challenge);
            }
        });
    }

    async function providerLoginPkce(id: string, pkce_challenge: string) {
        isLoading = true;

        let pow = (await fetchSolvePow()) || '';
        let payload: ProviderLoginRequest = {
            email: user.email,
            pow,
            client_id: 'rauthy',
            redirect_uri: window.location.href,
            provider_id: id,
            pkce_challenge,
        };

        let res = await fetchPost<string>(`/auth/v1/providers/${id}/link`, payload);
        isLoading = false;

        if (res.text) {
            saveProviderToken(res.text);
            let loc = res.headers.get('location');
            if (loc) {
                window.location.href = loc;
            }
        } else {
            console.error(res.error);
        }
    }

    async function unlinkProvider() {
        let res = await fetchDelete<UserResponse>('/auth/v1/providers/link');
        if (res.body) {
            user = res.body;
        } else {
            console.error(res.error);
            unlinkErr = true;
        }
    }
</script>

<Template id={TPL_IS_OTP_ENABLED} bind:value={isOtpEnabled} />

<div class="info-grid">
    <div class="row">
        <div class="admin-field-label">{t.common.email}</div>
        <span class="value">{user.email}</span>
    </div>

    {#if pamUser}
        <div class="row">
            <div class="admin-field-label">PAM {t.account.user}</div>
            <span class="value">{pamUser.name}</span>
        </div>
    {/if}

    {#if user.given_name}
        <div class="row">
            <div class="admin-field-label">{t.account.givenName}</div>
            <span class="value">{user.given_name}</span>
        </div>
    {/if}

    {#if user.family_name}
        <div class="row">
            <div class="admin-field-label">{t.account.familyName}</div>
            <span class="value">{user.family_name}</span>
        </div>
    {/if}

    <div class="row">
        <div class="admin-field-label">{t.account.user} ID</div>
        <span class="value">{user.id}</span>
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.accType}</div>
        <div>
            <div class="value">{accType || ''}</div>
            {#if isFederated}
                <div class="fed-btn">
                    <Button ariaLabel={t.account.providerUnlink} level={3} onclick={unlinkProvider}>
                        {t.account.providerUnlink}
                    </Button>
                    {#if unlinkErr}
                        <div class="link-err value">
                            {t.account.providerUnlinkDesc}
                        </div>
                    {/if}
                </div>
            {:else if providers.length > 0}
                <Button level={2} onclick={() => (showModal = true)}>
                    {t.account.providerLink}
                </Button>
                <Modal bind:showModal>
                    <h3>{t.account.providerLink}</h3>
                    <p>{t.account.providerLinkDesc}</p>

                    <div class="providers">
                        {#each providers as provider (provider.id)}
                            <ButtonAuthProvider
                                ariaLabel={`${t.account.providerLink}: ${provider.name}`}
                                {provider}
                                onclick={linkProvider}
                                {isLoading}
                            />
                        {/each}
                    </div>
                </Modal>
            {/if}
        </div>
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.roles}</div>
        <span class="value">{user.roles.join(', ') || 'None'}</span>
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.groups}</div>
        <span class="value">{user.groups?.join(', ') || 'None'}</span>
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.mfaActivated}</div>
        <CheckIcon checked={!!user.webauthn_user_id || hasOtp} />
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.userEnabled}</div>
        <CheckIcon checked={user.enabled} />
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.emailVerified}</div>
        <CheckIcon checked={user.email_verified} />
    </div>

    {#if user.last_login}
        <div class="row">
            <div class="admin-field-label">{t.account.lastLogin}</div>
            <span class="value">{formatDateFromTs(user.last_login)}</span>
        </div>
    {/if}

    <div class="row">
        <div class="admin-field-label">{t.account.passwordExpiry}</div>
        <span class="value">
            {(user.password_expires && formatDateFromTs(user.password_expires)) || t.common.never}
        </span>
    </div>

    <div class="row">
        <div class="admin-field-label">{t.account.userCreated}</div>
        <span class="value">{formatDateFromTs(user.created_at)}</span>
    </div>

    {#if user.user_expires}
        <div class="row">
            <div class="admin-field-label">{t.account.userExpiry}</div>
            <span class="value">{formatDateFromTs(user.user_expires)}</span>
        </div>
    {/if}

    {#if webIdData}
        <div class="row">
            <div class="admin-field-label">WebID:</div>
            <span class="value">
                <a href={buildWebIdUri(user.id)} target="_blank">
                    {@html buildWebIdUri(user.id).replace('/auth/', '/auth/<wbr/>')}
                </a>
            </span>
        </div>
    {/if}
</div>

<style>
    .info-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 16rem), 1fr));
        gap: var(--ui-space);
    }

    .row {
        min-width: 0;
        padding-block: 0.5rem;
        border-bottom: 1px solid var(--ui-outline);
    }

    .fed-btn,
    .providers {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        margin-top: 0.5rem;
    }

    .link-err {
        color: hsl(var(--error));
    }

    .value {
        font-size: var(--ui-control-font-size);
        line-height: var(--ui-control-line-height);
        overflow-wrap: anywhere;
    }
</style>
