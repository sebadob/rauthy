<script lang="ts">
    import CheckIcon from "$lib5/CheckIcon.svelte";
    import {buildWebIdUri, formatDateFromTs, saveProviderToken} from "$utils/helpers";
    import Button from "$lib5/button/Button.svelte";
    import Modal from "$lib5/Modal.svelte";
    import {PKCE_VERIFIER_UPSTREAM} from "$utils/constants";
    import {useI18n} from "$state/i18n.svelte.js";
    import type {UserResponse} from "$api/types/user.ts";
    import type {AuthProvidersTemplate, AuthProviderTemplate} from "$api/templates/AuthProvider.ts";
    import type {WebIdResponse} from "$api/types/web_id.ts";
    import type {ProviderLoginRequest} from "$api/types/auth_provider.ts";
    import {fetchDelete, fetchPost} from "$api/fetch";
    import ButtonAuthProvider from "../ButtonAuthProvider.svelte";
    import UserPicture from "$lib/UserPicture.svelte";
    import {fetchSolvePow} from "$utils/pow";
    import {generatePKCE} from "$utils/pkce";

    let {
        user = $bindable(),
        providers,
        authProvider,
        webIdData,
        viewModePhone,
    }: {
        user: UserResponse,
        providers: AuthProvidersTemplate,
        authProvider: undefined | AuthProviderTemplate,
        viewModePhone?: boolean,
        webIdData: undefined | WebIdResponse,
    } = $props();

    let t = useI18n();

    let unlinkErr = $state(false);
    let showModal = $state(false);

    let isFederated = $derived(user.account_type?.startsWith('federated'));
    let accType = $derived(isFederated ? `${user.account_type}: ${authProvider?.name || ''}` : user.account_type);

    let classRow: 'rowPhone' | 'row' = $derived(viewModePhone ? 'rowPhone' : 'row');
    let classLabel: 'labelPhone' | 'label' = $derived(viewModePhone ? 'labelPhone' : 'label');

    let isLoading = $state(false);

    let fallbackCharacters = $derived.by(() => {
        let chars = user.given_name[0];
        if (user.family_name && user.family_name.length > 0) {
            chars += user.family_name[0];
        }
        return chars;
    });

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

        let pow = await fetchSolvePow() || '';
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

<div>
    <div class={classRow} style:margin=".5rem 0">
        <div class={classLabel}></div>
        <UserPicture
                {fallbackCharacters}
                userId={user.id}
                bind:pictureId={user.picture_id}
                size="large"
        />
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.common.email}</div>
        <span class="value">{user.email}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.givenName}</div>
        <span class="value">{user.given_name}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.familyName}</div>
        <span class="value">{user.family_name}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.user} ID</div>
        <span class="value">{user.id}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.accType}</div>
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
                <Button level={2} onclick={() => showModal = true}>
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

    <div class={classRow}>
        <div class={classLabel}>{t.account.roles}</div>
        <span class="value">{user.roles || 'None'}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.groups}</div>
        <span class="value">{user.groups || 'None'}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.mfaActivated}</div>
        <CheckIcon checked={!!user.webauthn_user_id}/>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.userEnabled}</div>
        <CheckIcon checked={user.enabled}/>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.emailVerified}</div>
        <CheckIcon checked={user.email_verified}/>
    </div>

    {#if user.last_login}
        <div class={classRow}>
            <div class={classLabel}>{t.account.lastLogin}</div>
            <span class="value">{formatDateFromTs(user.last_login)}</span>
        </div>
    {/if}

    <div class={classRow}>
        <div class={classLabel}>{t.account.passwordExpiry}</div>
        <span class="value">
            {user.password_expires && formatDateFromTs(user.password_expires) || t.common.never}
        </span>
    </div>

    <div class={classRow}>
        <div class={classLabel}>{t.account.userCreated}</div>
        <span class="value">{formatDateFromTs(user.created_at)}</span>
    </div>

    {#if user.user_expires}
        <div class={classRow}>
            <div class={classLabel}>{t.account.userExpiry}</div>
            <span class="value">{formatDateFromTs(user.user_expires)}</span>
        </div>
    {/if}

    {#if webIdData}
        <div class={classRow}>
            <div class={classLabel}>WebID:</div>
            <span class="value">
                <a href={buildWebIdUri(user.id)} target="_blank">
                    {@html buildWebIdUri(user.id).replace('/auth/', '/auth/<wbr/>')}
                </a>
            </span>
        </div>
    {/if}
</div>

<style>
    .label, .labelPhone {
        color: hsla(var(--text) / .66);
        font-size: .9rem;
    }

    .label {
        margin-top: -.05rem;
        width: 8.5rem;
    }

    .labelPhone {
        width: 12rem;
    }

    .fed-btn {
        margin-left: -5px;
    }

    .link-err {
        margin-left: 5px;
        color: hsl(var(--error));
    }

    .providers {
        margin-top: 1rem;
        display: flex;
    }

    .row {
        display: flex;
        align-items: center;
        margin-bottom: .25rem;
    }

    .rowPhone {
        margin-bottom: .25rem;
    }

    .rowPhone > .labelPhone {
        margin-bottom: -.4rem;
    }

    .value {
        max-width: 18rem;
        word-break: break-word;
    }
</style>
