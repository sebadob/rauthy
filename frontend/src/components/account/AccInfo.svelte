<script>
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {formatDateFromTs} from "../../utils/helpers.js";

    export let user = {};

</script>

<div class="container">
    <div class="row">
        <div class="label"><b>E-Mail:</b></div>
        <span class="value">{user.email}</span>
    </div>

    <div class="row">
        <div class="label"><b>Given Name:</b></div>
        <span class="value">{user.given_name}</span>
    </div>

    <div class="row">
        <div class="label"><b>Family Name:</b></div>
        <span class="value">{user.family_name}</span>
    </div>

    <div class="row">
        <div class="label"><b>User ID:</b></div>
        <span class="value">{user.id}</span>
    </div>

    <div class="row">
        <div class="label"><b>Roles:</b></div>
        <span class="value">{user.roles || 'None'}</span>
    </div>

    <div class="row">
        <div class="label"><b>Groups:</b></div>
        <span class="value">{user.groups || 'None'}</span>
    </div>

    <div class="row">
        <div class="label"><b>MFA activated:</b></div>
        <CheckIcon check={user.mfa_app || user.sec_key_1 || user.sec_key_2}/>
    </div>

    <div class="row">
        <div class="label"><b>User enabled:</b></div>
        <CheckIcon check={user.enabled}/>
    </div>

    <div class="row">
        <div class="label"><b>E-Mail verified:</b></div>
        <CheckIcon check={user.email_verified}/>
    </div>

    <div class="row">
        <div class="label"><b>Last Login:</b></div>
        <span class="value">{formatDateFromTs(user.last_login)}</span>
    </div>

    <div class="row">
        <div class="label"><b>Password expiry:</b></div>
        <span class="value">{user.password_expires && formatDateFromTs(user.password_expires) || 'Never'}</span>
    </div>

    <div class="row">
        <div class="label"><b>User created at:</b></div>
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
