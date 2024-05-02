<script>
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {buildWebIdUri, formatDateFromTs} from "../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {deleteUserProviderLink} from "../../utils/dataFetching.js";
    import Modal from "$lib/Modal.svelte";
    import getPkce from "oauth-pkce";
    import {PKCE_VERIFIER_UPSTREAM} from "../../utils/constants.js";

    export let t;
    export let user = {};
    // webIdData will stay undefined if it is not enabled in the backend
    export let authProvider;
    export let webIdData;
    export let viewModePhone = false;

    let unlinkErr = false;
    let showModal = true;
    // let showModal = false;
    let providersAvailable = [];

    $: isFederated = user.account_type.startsWith('federated');
    $: accType = isFederated ? `${user.account_type}: ${authProvider?.name || ''}` : user.account_type;

    $: classRow = viewModePhone ? 'rowPhone' : 'row';
    $: classLabel = viewModePhone ? 'labelPhone' : 'label';

    onMount(() => {
        // value for dev testing only
        const providersTpl = [{
            "id": "7F6N7fb3el3P5XimjJSaeD2o",
            "name": "Rauthy IAM"
        }, {"id": "7F6N7fb3el3P5XimjJSaeD2O", "name": "Rauthy IAM 2"}];
        // const providersTpl = document?.getElementsByTagName("template").namedItem("auth_providers")?.innerHTML;
        console.log(providersTpl);
        providersAvailable = providersTpl;
    })

    function linkProvider(id) {
        console.error('TODO linkProvider');
        // getPkce(64, (error, {challenge, verifier}) => {
        //     if (!error) {
        //         localStorage.setItem(PKCE_VERIFIER_UPSTREAM, verifier);
        //         providerLoginPkce(id, challenge);
        //     }
        // });
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
        <div class={classLabel}><b>{t.email}:</b></div>
        <span class="value">{user.email}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.givenName}:</b></div>
        <span class="value">{user.given_name}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.familyName}:</b></div>
        <span class="value">{user.family_name}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.user} ID:</b></div>
        <span class="value">{user.id}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.accType}:</b></div>
        <div>
            <div class="value">{accType || ''}</div>
            {#if isFederated}
                <div class="fed-btn">
                    <Button level={3} on:click={unlinkProvider}>
                        {t.providerUnlink}
                    </Button>
                    {#if unlinkErr}
                        <div class="link-err value">
                            {t.providerUnlinkDesc}
                        </div>
                    {/if}
                </div>
            {:else if providersAvailable.length > 0}
                <div
                        role="button"
                        tabindex="0"
                        class="provider-link"
                        on:click={() => showModal = !showModal}
                        on:keypress={() => showModal = !showModal}
                >
                    {t.providerLink}
                </div>
                <Modal bind:showModal>
                    <p>
                        {t.providerLinkDesc}
                    </p>

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
        <div class={classLabel}><b>{t.roles}:</b></div>
        <span class="value">{user.roles || 'None'}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.groups}:</b></div>
        <span class="value">{user.groups || 'None'}</span>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.mfaActivated}:</b></div>
        <CheckIcon check={!!user.webauthn_user_id}/>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.user} {t.enabled}:</b></div>
        <CheckIcon check={user.enabled}/>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.emailVerified}:</b></div>
        <CheckIcon check={user.email_verified}/>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.lastLogin}:</b></div>
        <span class="value">{formatDateFromTs(user.last_login)}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.passwordExpiry}:</b></div>
        <span class="value">{user.password_expires && formatDateFromTs(user.password_expires) || t.never}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.user} {t.created}:</b></div>
        <span class="value">{formatDateFromTs(user.created_at)}</span>
    </div>

    {#if user.user_expires}
        <div class={classRow}>
            <div class={classLabel}><b>{t.userExpiry}:</b></div>
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

    dialog {
        animation: fade-out 0.7s ease-out;
    }

    dialog[open] {
        animation: fade-in 0.7s ease-out;
    }

    dialog[open]::backdrop {
        animation: backdrop-fade-in 0.7s ease-out forwards;
    }

    @keyframes fade-in {
        0% {
            opacity: 0;
            transform: scaleY(0);
            display: none;
        }

        100% {
            opacity: 1;
            transform: scaleY(1);
            display: block;
        }
    }

    @keyframes fade-out {
        0% {
            opacity: 1;
            transform: scaleY(1);
            display: block;
        }

        100% {
            opacity: 0;
            transform: scaleY(0);
            display: none;
        }
    }

    @keyframes backdrop-fade-in {
        0% {
            background-color: rgb(0 0 0 / 0%);
        }

        100% {
            background-color: rgb(0 0 0 / 25%);
        }
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
