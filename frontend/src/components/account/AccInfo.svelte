<script>
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {buildWebIdUri, formatDateFromTs} from "../../utils/helpers.js";
    import {onMount} from "svelte";
    import {getAuthProvidersTemplate} from "../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import {deleteUserProviderLink} from "../../utils/dataFetching.js";

    export let t;
    export let user = {};
    // webIdData will stay undefined if it is not enabled in the backend
    export let authProvider;
    export let webIdData;
    export let viewModePhone = false;

    let unlinkErr = false;

    $: isFederated = user.account_type.startsWith('federated');
    $: accType = isFederated ? `${user.account_type}: ${authProvider?.name || ''}` : user.account_type;

    $: classRow = viewModePhone ? 'rowPhone' : 'row';
    $: classLabel = viewModePhone ? 'labelPhone' : 'label';

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
            <div class="value">{accType}</div>
            {#if isFederated}
                <div class="fed-btn">
                    <Tooltip html={t.providerUnlinkDesc}>
                        <Button level={3} on:click={unlinkProvider}>
                            {t.providerUnlink}
                        </Button>
                    </Tooltip>
                </div>
            {:else}
                <!-- TODO -->
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

    .label {
        width: 10rem;
    }

    .labelPhone {
        width: 12rem;
    }

    .fed-btn {
        margin-left: -5px;
    }

    .row {
        display: flex;
    }

    .value {
        max-width: 18rem;
        word-break: break-word;
    }
</style>
