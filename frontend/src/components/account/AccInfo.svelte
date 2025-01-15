<script>
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {buildWebIdUri, formatDateFromTs, saveProviderToken} from "../../utils/helpers";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {deleteUserProviderLink, postUserProviderLink} from "../../utils/dataFetching.js";
    import Modal from "$lib/Modal.svelte";
    import getPkce from "oauth-pkce";
    import {PKCE_VERIFIER_UPSTREAM} from "../../utils/constants.js";
    import {useI18n} from "$state/i18n.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [user]
     * @property {any} authProvider - webIdData will stay undefined if it is not enabled in the backend
     * @property {any} webIdData
     * @property {boolean} [viewModePhone]
     */

    /** @type {Props} */
    let {
        user = $bindable({}),
        authProvider,
        webIdData,
        viewModePhone = false
    } = $props();

    let t = useI18n();

    let unlinkErr = $state(false);
    let showModal = $state(false);
    let providersAvailable = $state([]);

    let isFederated = $derived(user.account_type?.startsWith('federated'));
    let accType = $derived(isFederated ? `${user.account_type}: ${authProvider?.name || ''}` : user.account_type);

    let classRow = $derived(viewModePhone ? 'rowPhone' : 'row');
    let classLabel = $derived(viewModePhone ? 'labelPhone' : 'label');

    onMount(() => {
        // value for dev testing only
        // let tpl = '[{"id": "7F6N7fb3el3P5XimjJSaeD2o", "name": "Rauthy IAM"}]';
        let tpl = document?.getElementsByTagName("template").namedItem("auth_providers")?.innerHTML;
        // the additional comparison is just for local dev and no big deal in prod
        if (tpl && tpl !== '{{ auth_providers|safe }}') {
            providersAvailable = JSON.parse(tpl);
        }
    })

    function linkProvider(id) {
        getPkce(64, (error, {challenge, verifier}) => {
            if (!error) {
                localStorage.setItem(PKCE_VERIFIER_UPSTREAM, verifier);
                providerLoginPkce(id, challenge);
            }
        });
    }

    async function providerLoginPkce(id, pkce_challenge) {
        let data = {
            email: user.email,
            client_id: 'rauthy',
            redirect_uri: window.location.href,
            // scopes: '',
            // state: state,
            // nonce: nonce,
            // code_challenge: challenge,
            // code_challenge_method: challengeMethod,
            provider_id: id,
            pkce_challenge,
        };
        let res = await postUserProviderLink(id, data);
        if (res.ok) {
            const xsrfToken = await res.text();
            saveProviderToken(xsrfToken);

            window.location.href = res.headers.get('location');
        } else {
            let body = await res.json();
            // TODO catch error even necessary? should be handled in `/callback` already...
            console.error(body);
        }
    }

    async function unlinkProvider() {
        let res = await deleteUserProviderLink();
        let body = await res.json();
        if (res.ok) {
            user = body;
        } else {
            unlinkErr = true;
        }
    }

</script>

<div class="container">
    <div class={classRow}>
        <div class={classLabel}><b>{t.common.email}:</b></div>
        <span class="value">{user.email}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.givenName}:</b></div>
        <span class="value">{user.given_name}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.familyName}:</b></div>
        <span class="value">{user.family_name}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.user} ID:</b></div>
        <span class="value">{user.id}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.accType}:</b></div>
        <div>
            <div class="value">{accType || ''}</div>
            {#if isFederated}
                <div class="fed-btn">
                    <Button level={3} on:click={unlinkProvider}>
                        {t.account.providerUnlink}
                    </Button>
                    {#if unlinkErr}
                        <div class="link-err value">
                            {t.account.providerUnlinkDesc}
                        </div>
                    {/if}
                </div>
            {:else if providersAvailable.length > 0}
                <div
                        role="button"
                        tabindex="0"
                        class="provider-link"
                        onclick={() => showModal = !showModal}
                        onkeypress={() => showModal = !showModal}
                >
                    {t.account.providerLink}
                </div>
                <Modal bind:showModal>
                    <p>{t.account.providerLinkDesc}</p>

                    <div class="providers">
                        {#each providersAvailable as provider (provider.id)}
                            <Button on:click={() => linkProvider(provider.id)} level={3}>
                                <div class="flex-inline">
                                    <img
                                            src="{`/auth/v1/providers/${provider.id}/img`}"
                                            alt=""
                                            width="20"
                                            height="20"
                                    />
                                    <span class="provider-name">
                                        {provider.name}
                                    </span>
                                </div>
                            </Button>
                        {/each}
                    </div>
                </Modal>
            {/if}
        </div>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.roles}:</b></div>
        <span class="value">{user.roles || 'None'}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.groups}:</b></div>
        <span class="value">{user.groups || 'None'}</span>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.account.mfaActivated}:</b></div>
        <CheckIcon check={!!user.webauthn_user_id}/>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.account.userEnabled}:</b></div>
        <CheckIcon check={user.enabled}/>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.account.emailVerified}:</b></div>
        <CheckIcon check={user.email_verified}/>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.lastLogin}:</b></div>
        <span class="value">{formatDateFromTs(user.last_login)}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.passwordExpiry}:</b></div>
        <span class="value">
            {user.password_expires && formatDateFromTs(user.password_expires) || t.common.never}
        </span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.account.userCreated}:</b></div>
        <span class="value">{formatDateFromTs(user.created_at)}</span>
    </div>

    {#if user.user_expires}
        <div class={classRow}>
            <div class={classLabel}><b>{t.account.userExpiry}:</b></div>
            <span class="value">{formatDateFromTs(user.user_expires)}</span>
        </div>
    {/if}

    {#if webIdData}
        <div class={classRow}>
            <div class={classLabel}><b>WebID:</b></div>
            <span class="value">
                <a href={buildWebIdUri(user.id)} target="_blank">
                    {@html buildWebIdUri(user.id).replace('/auth/', '/auth/<wbr/>')}
                </a>
            </span>
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0 .25rem;
        padding: 10px;
    }

    .label {
        width: 10rem;
    }

    .labelPhone {
        width: 12rem;
    }

    .fed-btn {
        margin-left: -5px;
    }

    .flex-inline {
        display: inline-flex;
        align-items: center;
        gap: .5rem;
    }

    .link-err {
        margin-left: 5px;
        color: var(--col-err);
    }

    .provider-link {
        color: var(--col-act2);
        cursor: pointer;
    }

    .provider-name {
        margin-bottom: -4px;
    }

    .providers {
        margin-top: .66rem;
        display: flex;
    }

    .row {
        display: flex;
    }

    .value {
        max-width: 18rem;
        word-break: break-word;
    }
</style>
