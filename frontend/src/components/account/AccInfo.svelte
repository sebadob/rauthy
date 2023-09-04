<script>
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {formatDateFromTs} from "../../utils/helpers.js";

    export let t;
    export let user = {};
    export let viewModePhone = false;

    $: classRow = viewModePhone ? 'rowPhone' : 'row';
    $: classLabel = viewModePhone ? 'labelPhone' : 'label';

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
        <div class={classLabel}><b>{t.roles}:</b></div>
        <span class="value">{user.roles || 'None'}</span>
    </div>

    <div class={classRow}>
        <div class={classLabel}><b>{t.groups}:</b></div>
        <span class="value">{user.groups || 'None'}</span>
    </div>

    <div class="row">
        <div class={classLabel}><b>{t.mfaActivated}:</b></div>
        <CheckIcon check={user.webauthn_enabled}/>
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
</div>

<style>
    .container {
        padding: 10px;
    }

    .label {
        width: 10rem;
    }

    .labelPhone {
        width: 12rem;
    }

    .row {
        display: flex;
    }

    .value {
    }
</style>
