<script>
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {formatDateFromTs} from "../../utils/helpers.js";

    export let t;
    export let user = {};

</script>

<div class="container">
    <div class="row">
        <div class="label"><b>{t.email}:</b></div>
        <span class="value">{user.email}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.givenName}:</b></div>
        <span class="value">{user.given_name}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.familyName}:</b></div>
        <span class="value">{user.family_name}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.user} ID:</b></div>
        <span class="value">{user.id}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.roles}:</b></div>
        <span class="value">{user.roles || 'None'}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.groups}:</b></div>
        <span class="value">{user.groups || 'None'}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.mfaActivated}:</b></div>
        <CheckIcon check={user.mfa_app || user.sec_key_1 || user.sec_key_2}/>
    </div>

    <div class="row">
        <div class="label"><b>{t.user} {t.enabled}:</b></div>
        <CheckIcon check={user.enabled}/>
    </div>

    <div class="row">
        <div class="label"><b>{t.emailVerified}:</b></div>
        <CheckIcon check={user.email_verified}/>
    </div>

    <div class="row">
        <div class="label"><b>{t.lastLogin}:</b></div>
        <span class="value">{formatDateFromTs(user.last_login)}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.passwordExpiry}:</b></div>
        <span class="value">{user.password_expires && formatDateFromTs(user.password_expires) || t.never}</span>
    </div>

    <div class="row">
        <div class="label"><b>{t.user} {t.created}:</b></div>
        <span class="value">{formatDateFromTs(user.created_at)}</span>
    </div>
</div>

<style>
    .container {
        padding: 10px;
    }

    .label {
        width: 130px;
    }

    .row {
        display: flex;
    }

    .value {
        width: 210px;
    }
</style>
