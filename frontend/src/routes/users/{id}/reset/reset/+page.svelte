<script>
    import * as yup from "yup";
    import {onMount} from "svelte";
    import {extractFormErrors, generatePassword} from "../../../../../utils/helpers.js";
    import {resetPassword, webauthnAuthStart} from "../../../../../utils/dataFetching.js";
    import Loading from "$lib/Loading.svelte";
    import Button from "$lib/Button.svelte";
    import PasswordPolicy from "../../../../../components/passwordReset/PasswordPolicy.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import WebauthnRequest from "../../../../../components/webauthn/WebauthnRequest.svelte";
    import BrowserCheck from "../../../../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";

    const btnWidth = 150;
    const inputWidth = '320px';

    let t;
    let csrf = '';
    let policy;
    let isReady = false;
    let isMfa = false;

    let isLoading = false;
    let err = '';
    let userId = '';
    let success = false;
    let accepted = false;
    let showCopy = false;
    let webauthnData;

    let formValues = {
        email: '',
        password: '',
        passwordConfirm: '',
    };
    let formErrors = {};

    let schema;
    $: if (t) {
        schema = yup.object().shape({
            email: yup.string().required(t.required).email(t.badFormat),
            password: yup.string().required(t.required),
            passwordConfirm: yup.string().required(t.required)
        });
    }

    $: if (formValues.password?.length > 0 && formValues.password === formValues.passwordConfirm) {
        showCopy = true;
    }

    $: if (success) {
        setTimeout(() => {
            window.location.replace('/auth/v1/account');
        }, 5000);
    }

    onMount(async () => {
        // const policy_vals = '10, 128, 1, 1, 1, 1, 3';
        const policy_vals = document.getElementsByName('rauthy-data')[0].id;
        const arr = [];
        policy_vals
            .split(',')
            .forEach(v => arr.push(v));
        // policy = {
        // 	length_min: arr[0],
        // 	length_max: arr[1],
        // 	include_lower_case: arr[2],
        // 	include_upper_case: arr[3],
        // 	include_digits: arr[4],
        // 	include_special: arr[5],
        // 	not_recently_used: arr[6],
        // };
        policy = {
            length_min: Number.parseInt(arr[0]),
            length_max: Number.parseInt(arr[1]),
            include_lower_case: Number.parseInt(arr[2]),
            include_upper_case: Number.parseInt(arr[3]),
            include_digits: Number.parseInt(arr[4]),
            include_special: Number.parseInt(arr[5]),
            not_recently_used: Number.parseInt(arr[6]),
        };
        let email = arr[7];
        if (email && email !== "undefined") {
            formValues.email = email;
            isMfa = true;
        }

        csrf = window.document.getElementsByName('rauthy-csrf-token')[0].id
        userId = window.location.href.split("/users/")[1].split("/")[0];
        isReady = true;
    })

    function generate() {
        const len = policy.length_min > 24 ? policy.length_min : 24;
        let pwd = generatePassword(
            len, policy.include_lower_case,
            policy.include_upper_case,
            policy.include_digits,
            policy.include_special,
        );
        formValues.password = pwd;
        formValues.passwordConfirm = pwd;
    }

    async function onSubmit() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        if (!accepted) {
            return;
        }

        // do passwords match?
        if (formValues.password !== formValues.passwordConfirm) {
            err = t.passwordNoMatch;
            return;
        } else {
            err = '';
        }

        if (isMfa) {
            let res = await webauthnAuthStart(userId, {purpose: 'PasswordReset'});
            let body = await res.json();
            if (!res.ok) {
                err = body.message;
                isLoading = false;
                return;
            }

            if (body.user_id !== userId) {
                err = 'MFA user ID does not match - this should never happen';
                isLoading = false;
                return;
            }

            webauthnData = body;
        } else {
            await onSubmitFinish();
        }
    }

    async function onSubmitFinish(mfaCode) {
        isLoading = true;

        const magicLinkId = window.location.href.split("/reset/")[1];
        const data = {
            email: formValues.email,
            password: formValues.password,
            magic_link_id: magicLinkId,
            mfa_code: mfaCode,
        };

        const res = await resetPassword(userId, data, csrf);
        if (res.ok) {
            err = '';
            success = true;
        } else {
            const body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    function onWebauthnError() {
        // If there is any error with the key, the user should start a new login process
        webauthnData = undefined;
    }

    function onWebauthnSuccess(res) {
        if (res) {
            webauthnData = undefined;
            onSubmitFinish(res.code)
        }
    }

</script>

<svelte:head>
    <title>Password Reset</title>
</svelte:head>

<BrowserCheck>
    {#if !isReady}
        <Loading/>
    {/if}

    <WithI18n bind:t content="passwordReset">
        {#if webauthnData}
            <WebauthnRequest
                    bind:data={webauthnData}
                    purpose="PasswordReset"
                    onSuccess={onWebauthnSuccess}
                    onError={onWebauthnError}
            />
        {/if}

        <div class="container">
            <h1>Password Reset</h1>

            <PasswordPolicy bind:t bind:accepted bind:policy bind:password={formValues.password}/>

            <Input
                    type="email"
                    bind:value={formValues.email}
                    bind:error={formErrors.email}
                    autocomplete="email"
                    disabled={isMfa}
                    placeholder={t.email}
                    width={inputWidth}
            >
                {t.email.toUpperCase()}
            </Input>
            <PasswordInput
                    bind:value={formValues.password}
                    bind:error={formErrors.password}
                    autocomplete="new-password"
                    placeholder={t.password}
                    width={inputWidth}
                    bind:showCopy
            >
                {t.password.toUpperCase()}
            </PasswordInput>
            <PasswordInput
                    bind:value={formValues.passwordConfirm}
                    bind:error={formErrors.passwordConfirm}
                    autocomplete="new-password"
                    placeholder={t.passwordConfirm}
                    width={inputWidth}
                    bind:showCopy
            >
                {t.passwordConfirm.toUpperCase()}
            </PasswordInput>

            <Button on:click={generate} width={btnWidth} level={3}>
                {t.generate.toUpperCase()}
            </Button>
            <Button on:click={onSubmit} width={btnWidth} bind:isLoading level={2}>
                {t.save.toUpperCase()}
            </Button>

            {#if success}
                <div class="success">
                    {t.success1}
                    <br>
                    {t.success2}
                </div>
            {:else if err}
                <div class="err">
                    {err}
                </div>
            {/if}
        </div>

        <LangSelector absolute />
    </WithI18n>
</BrowserCheck>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .err {
        margin: 0 5px;
        color: var(--col-err);
    }

    .success {
        margin: 5px;
        color: var(--col-acnt);
    }
</style>
