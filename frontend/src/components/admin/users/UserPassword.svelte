<script>
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import PasswordPolicy from "../../passwordReset/PasswordPolicy.svelte";
    import {onMount} from "svelte";
    import {getPasswordPolicy} from "../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import {postPasswordResetRequest, putUser} from "../../../utils/dataFetchingAdmin.js";
    import {generatePassword} from "../../../utils/helpers.js";

    export let user = {};
    export let onSave;

    const btnWidth = "inherit";

    let isLoading = false;
    let errEmail = '';
    let errPwd = '';
    let successEmail = false;
    let successPwd = false;
    let timer;
    let pwdWith = '330px';

    let policy;
    let accepted;

    let formValues = {new: '', verify: ''};
    let formErrors = {};

    $: if (successPwd) {
        timer = setTimeout(() => {
            successPwd = false;
            onSave();
        }, 3000);
    }

    onMount(async () => {
        if (!policy) {
            let res = await getPasswordPolicy();
            let body = await res.json();
            if (!res.ok) {
                errPwd = body.message;
            } else {
                policy = body;
            }
        }
    });

    async function isPolicyValid() {
        errPwd = '';

        let formValid = await isFormValid();
        if (!formValid) {
            return false;
        }

        if (!accepted) {
            errPwd = 'You must follow the password policy';
            return false;
        }

        return true;
    }

    async function isFormValid() {
        errPwd = '';

        if (formValues.new.length > 256) {
            errPwd = 'max 256 characters';
            return false;
        }

        if (formValues.new !== formValues.verify) {
            errPwd = 'New passwords do not match';
            return false;
        }

        return true;
    }

    async function sendEmail() {
        let req = {
            email: user.email,
        };

        errEmail = '';
        isLoading = true;

        let res = await postPasswordResetRequest(req);
        if (res.ok) {
            successEmail = true;
        } else {
            let body = await res.json();
            errEmail = body.message;
        }

        isLoading = false;
    }

    async function savePwd() {
        let req = {
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            roles: user.roles,
            groups: user.groups,
            enabled: user.enabled,
            email_verified: user.email_verified,
        };

        if (formValues.new.length > 0) {
            const valid = await isFormValid() && await isPolicyValid();
            if (!valid) {
                return;
            }

            req.password = formValues.new;
        }

        errPwd = '';
        isLoading = true;

        let res = await putUser(user.id, req);
        if (res.ok) {
            successPwd = true;
        } else {
            let body = await res.json();
            errPwd = body.message;
        }

        isLoading = false;
    }

    function generate() {
        const len = policy.length_min > 24 ? policy.length_min : 24;
        let pwd = generatePassword(
            len, policy.include_lower_case, policy.include_upper_case, policy.include_digits, policy.include_special
        );
        formValues.new = pwd;
        formValues.verify = pwd;
    }

</script>

<div class="container">
    {#if user.account_type === "new"}
        <div class="desc">
            <p><b>The user has not initialized this account yet.</b></p>
            <p>You may send out a new Reset E-Mail, if the user has not received one.</p>
        </div>

        <Button
                on:click={sendEmail}
                bind:isLoading
                width={btnWidth}
                level={2}
        >
            SEND RESET E-MAIL
        </Button>
    {:else if user.account_type === "passkey"}
        <div class="desc">
            <p><b>This is a passkey only account type.</b></p>
            <p>
                This means, that this user is using the passwordless login flow only and does not have any password.
            </p>
            <p>
                If the user has lost all his keys and you have verified everything, you may reset his account type and
                send out a new reset E-Mail, so the user can get access again.
            </p>
            <p>
                To reset the account, navigate to 'MFA' and delete all registered keys for this user.
            </p>
        </div>
    {:else}
        {#if policy}
            <PasswordPolicy bind:password={formValues.new} bind:accepted bind:policy/>
        {/if}

        <PasswordInput
                type="password"
                bind:value={formValues.new}
                on:blur={isFormValid}
                bind:width={pwdWith}
                autocomplete="off"
                showCopy={formValues.new.length > 0 && formValues.new === formValues.verify}
        >
            New Password
        </PasswordInput>
        <PasswordInput
                type="password"
                bind:value={formValues.verify}
                on:blur={isFormValid}
                bind:width={pwdWith}
                autocomplete="off"
        >
            New Password
        </PasswordInput>

        <Button on:click={generate} width={btnWidth} level={3}>
            GENERATE RANDOM
        </Button>

        <div class="desc">
            You can either set and reset a user's password<br/>
            or send out a new reset E-Mail for self-service.
        </div>

        <Button
                on:click={sendEmail}
                bind:isLoading
                width={btnWidth}
                level={1}
        >
            SEND RESET E-MAIL
        </Button>

        <Button
                on:click={savePwd}
                bind:isLoading
                width={btnWidth}
        >
            SAVE PASSWORD
        </Button>
    {/if}

    {#if successPwd || successEmail}
        <div class="success">
            Success
        </div>
    {/if}

    {#if errPwd}
        <div class="err">
            {errPwd}
        </div>
    {/if}

    {#if errEmail}
        <div class="err">
            {errEmail}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0 10px 10px 10px;
    }

    .desc {
        margin: 7px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .success {
        color: var(--col-ok);
    }
</style>
