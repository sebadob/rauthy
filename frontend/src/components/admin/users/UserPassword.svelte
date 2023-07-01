<script>
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import PasswordPolicy from "../../passwordReset/PasswordPolicy.svelte";
    import {onMount} from "svelte";
    import {getPasswordPolicy, webauthnDelete} from "../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import {postPasswordResetRequest, putUser} from "../../../utils/dataFetchingAdmin.js";
    import {generatePassword} from "../../../utils/helpers.js";

    export let user = {};
    export let onSave;

    const btnWidth = 160;

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

    async function handleDeleteSlot(slot) {
        let res = await webauthnDelete(user.id, slot);
        if (res.status === 200) {
            return true;
        } else {
            let body = await res.json();
            if (body.message !== 'No key registered in this slot') {
                errPwd = body.message;
            }
            return false;
        }
    }

    function generate() {
        const len = policy.length_min > 24 ? policy.length_min : 24;
        let pwd = generatePassword(
            len, policy.include_lower_case, policy.include_upper_case, policy.include_digits, policy.include_special
        );
        formValues.new = pwd;
        formValues.verify = pwd;
    }

    async function resetMfa() {
        // ignore the results, just fire the request and skip checking, if a key in these slots does exist -> more efficient
        await handleDeleteSlot(1);
        await handleDeleteSlot(2);
        successPwd = true;
    }

</script>

<div class="container">
    {#if policy}
        <PasswordPolicy bind:password={formValues.new} bind:accepted bind:policy/>
    {/if}

    <PasswordInput
            bind:value={formValues.new}
            on:blur={isFormValid}
            bind:width={pwdWith}
            autocomplete="off"
            showCopy={formValues.new.length > 0 && formValues.new === formValues.verify}
    >
        New Password
    </PasswordInput>
    <PasswordInput
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
        You can reset the users MFA / Security Keys.<br>
        Be careful though, since this <b>cannot be reverted</b> without user interaction.<br>
        It will disable all MFA logins for this user.
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

    <Button
            on:click={resetMfa}
            bind:isLoading
            width={btnWidth}
            level={3}
    >
        RESET MFA
    </Button>

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
