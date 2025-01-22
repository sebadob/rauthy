<script lang="ts">
    import {run} from 'svelte/legacy';
    import * as yup from "yup";
    import {onMount} from "svelte";
    import {
        arrBufToBase64UrlSafe,
        base64UrlSafeToArrBuf,
        extractFormErrors,
        generatePassword,
        getQueryParams
    } from "../../../../../utils/helpers";
    import {
        resetPassword,
        webauthnAuthStart,
        webauthnRegStartAccReset,
        webauthnRegFinishAccReset,
    } from "../../../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import PasswordPolicy from "../../../../../components/passwordReset/PasswordPolicy.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import WebauthnRequest from "../../../../../components/webauthn/WebauthnRequest.svelte";
    import {slide} from "svelte/transition";
    import LangSelector from "$lib5/LangSelector.svelte";
    import {REGEX_NAME, TPL_PASSWORD_RESET} from "../../../../../utils/constants.js";
    import {useIsDev} from "$state/is_dev.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import type {PasswordResetTemplate} from "$api/templates/PasswordReset";
    import Template from "$lib5/Template.svelte";
    import {useParam} from "$state/param.svelte.ts";

    const btnWidth = '150px';
    const inputWidth = '320px';

    let t = useI18n();

    let data: undefined | PasswordResetTemplate = $state();

    let isLoading = $state(false);
    let err = $state('');
    let requestType = $state('');
    let accountTypeNew = $state('');
    let redirectUri = $state('');
    let success = $state(false);
    let accepted = $state(false);
    let showCopy = $state(false);
    let webauthnData = $state();

    let formValues = $state({
        passkeyName: '',
        password: '',
        passwordConfirm: '',
    });
    let formErrors = $state({});

    let schemaPasskey = $state();
    let schemaPassword = $state();

    onMount(async () => {
        let isDev = useIsDev();
        if (isDev) {
            requestType = 'password_reset';
        } else {
            requestType = useParam('type').get() || 'password_reset';
        }
    })

    function navigateToAccount() {
        window.location.replace('/auth/v1/account');
    }

    function generate() {
        if (data) {
            const len = data.password_policy.length_min > 24 ? data.password_policy.length_min : 24;
            let pwd = generatePassword(
                len,
                data.password_policy.include_lower_case,
                data.password_policy.include_upper_case,
                data.password_policy.include_digits,
                data.password_policy.include_special,
            );
            formValues.password = pwd;
            formValues.passwordConfirm = pwd;
        }
    }

    async function handleRegisterPasskey() {
        if (!data) {
            console.error('template data is undefined');
            return;
        }

        err = '';

        try {
            await schemaPasskey.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        const passkeyName = formValues.passkeyName;
        if (passkeyName.length < 2) {
            err = t.mfa.passkeyNameErr;
            return;
        }

        let payload = {
            passkey_name: passkeyName,
            magic_link_id: data.magic_link_id,
        };
        let res = await webauthnRegStartAccReset(data.user_id, payload, data.csrf_token);
        if (res.status === 200) {
            let challenge = await res.json();

            // we need to force UV at this point in the browser already to have a better UV
            challenge.publicKey.authenticatorSelection.userVerification = 'required';

            // the navigator credentials engine needs some values as array buffers
            challenge.publicKey.challenge = base64UrlSafeToArrBuf(challenge.publicKey.challenge);
            challenge.publicKey.user.id = base64UrlSafeToArrBuf(challenge.publicKey.user.id);
            challenge.publicKey.excludeCredentials = challenge.publicKey.excludeCredentials

            if (challenge.publicKey.excludeCredentials) {
                challenge.publicKey.excludeCredentials = challenge.publicKey.excludeCredentials.map(cred => {
                    cred.id = base64UrlSafeToArrBuf(cred.id);
                    return cred;
                });
            }

            // prompt for the user security key and get its public key
            let challengePk = await navigator.credentials.create(challenge);

            // the backend expects base64 url safe string instead of array buffers
            let payload = {
                passkey_name: passkeyName,
                data: {
                    id: challengePk.id,
                    rawId: arrBufToBase64UrlSafe(challengePk.rawId),
                    response: {
                        attestationObject: arrBufToBase64UrlSafe(challengePk.response.attestationObject),
                        clientDataJSON: arrBufToBase64UrlSafe(challengePk.response.clientDataJSON),
                    },
                    type: challengePk.type,
                },
                magic_link_id: data.magic_link_id,
            }

            // send the keys' pk to the backend and finish the registration
            res = await webauthnRegFinishAccReset(userId, payload, data.csrf_token);
            if (res.status === 201) {
                formValues = {
                    passkeyName: '',
                    password: '',
                    passwordConfirm: '',
                };
                success = true;
            } else {
                onWebauthnError();
                console.error(res);
            }
        } else {
            onWebauthnError();
            let body = await res.json();
            console.error(body.error);
            console.error(body.message);
        }
    }

    async function passwordReset() {
        try {
            await schemaPassword.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        if (!accepted) {
            return;
        }

        if (formValues.password.length > 256) {
            err = 'max 256';
            return;
        }

        // do passwords match?
        if (formValues.password !== formValues.passwordConfirm) {
            err = t.passwordReset.passwordNoMatch;
            return;
        } else {
            err = '';
        }

        if (data?.needs_mfa) {
            let res = await webauthnAuthStart(data?.user_id, {purpose: 'PasswordReset'});
            let body = await res.json();
            if (!res.ok) {
                err = body.message;
                isLoading = false;
                return;
            }

            if (body.user_id !== data.user_id) {
                err = 'MFA user ID does not match - this should never happen';
                isLoading = false;
                return;
            }

            webauthnData = body;
        } else {
            await onSubmitFinish();
        }
    }

    async function onSubmitFinish(mfaCode?: string) {
        if (!data) {
            return;
        }

        isLoading = true;

        const payload = {
            password: formValues.password,
            magic_link_id: data.magic_link_id,
            mfa_code: mfaCode,
        };
        const res = await resetPassword(data.user_id, payload, data?.csrf_token);
        if (res.ok) {
            err = '';
            formValues = {
                passkeyName: '',
                password: '',
                passwordConfirm: '',
            };
            redirectUri = res.headers.get('Location');
            console.log('redirectUri: ' + redirectUri);
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
        err = t.mfa.errorReg;
    }

    function onWebauthnSuccess(res) {
        if (res) {
            webauthnData = undefined;
            onSubmitFinish(res.code)
        }
    }

    run(() => {
        if (t) {
            schemaPasskey = yup.object().shape({
                passkeyName: yup.string()
                    .required(t.common.required)
                    .matches(REGEX_NAME, t.mfa.passkeyNameErr),
            });
            schemaPassword = yup.object().shape({
                password: yup.string().required(t.common.required),
                passwordConfirm: yup.string().required(t.common.required)
            });
        }
    });
    run(() => {
        if (accountTypeNew) {
            // reset all possibly filled in form values from before
            formValues = {
                passkeyName: '',
                password: '',
                passwordConfirm: '',
            };
        }
    });
    run(() => {
        if (formValues.password?.length > 0 && formValues.password === formValues.passwordConfirm) {
            showCopy = true;
        }
    });
    run(() => {
        if (success) {
            setTimeout(() => {
                if (redirectUri) {
                    window.location.replace(redirectUri);
                } else {
                    navigateToAccount();
                }
            }, 5000);
        }
    });
</script>

<svelte:head>
    {#if t}
        {#if requestType.startsWith('new_user')}
            <title>{t.passwordReset.newAccount}</title>
        {:else if requestType === "password_reset"}
            <title>{t.passwordReset.passwordReset}</title>
        {/if}
    {:else}
        <title>Password</title>
    {/if}
</svelte:head>

<Template id={TPL_PASSWORD_RESET} bind:value={data}/>

<Main>
    <ContentCenter>
        {#if data}
            <div class="container">
                {#if requestType.startsWith('new_user')}
                    {#if webauthnData}
                        <WebauthnRequest
                                bind:data={webauthnData}
                                onSuccess={onWebauthnSuccess}
                                onError={onWebauthnError}
                        />
                    {/if}

                    <h1>{t.passwordReset.newAccount}</h1>
                    <p>{t.passwordReset.newAccDesc1}</p>
                    <p>{t.passwordReset.newAccDesc2}<a href={t.passwordReset.fidoLink} target="_blank">FIDO Alliance</a>
                    </p>

                    <div style:margin-bottom="1rem">
                        <Button
                                on:click={() => accountTypeNew = "passkey"}
                                width={btnWidth}
                                bind:isLoading
                                level={2}
                                isDisabled={success}
                        >
                            {t.passwordReset.passwordless}
                        </Button>
                        <Button
                                on:click={() => accountTypeNew = "password"}
                                width={btnWidth}
                                bind:isLoading
                                level={3}
                                isDisabled={success}
                        >
                            {t.passwordReset.password}
                        </Button>
                    </div>

                    {#if accountTypeNew === "password"}
                        <div transition:slide>
                            <PasswordPolicy bind:accepted policy={data.password_policy} password={formValues.password}/>

                            <PasswordInput
                                    bind:value={formValues.password}
                                    error={formErrors.password}
                                    autocomplete="new-password"
                                    placeholder={t.passwordReset.password}
                                    width={inputWidth}
                                    {showCopy}
                                    disabled={success}
                            >
                                {t.passwordReset.password.toUpperCase()}
                            </PasswordInput>
                            <PasswordInput
                                    bind:value={formValues.passwordConfirm}
                                    error={formErrors.passwordConfirm}
                                    autocomplete="new-password"
                                    placeholder={t.passwordReset.passwordConfirm}
                                    width={inputWidth}
                                    {showCopy}
                                    disabled={success}
                            >
                                {t.passwordReset.passwordConfirm.toUpperCase()}
                            </PasswordInput>

                            <Button
                                    on:click={generate}
                                    width={btnWidth}
                                    level={3}
                                    isDisabled={success}
                            >
                                {t.passwordReset.generate}
                            </Button>
                            <Button
                                    on:click={passwordReset}
                                    width={btnWidth}
                                    bind:isLoading level={2}
                                    isDisabled={success}
                            >
                                {t.common.save}
                            </Button>

                            {#if success}
                                <div class="success">
                                    {t.passwordReset.success1}
                                    <br>
                                    {t.passwordReset.success2}
                                </div>
                            {/if}
                        </div>
                    {:else if accountTypeNew === "passkey"}
                        <div transition:slide>
                            <Input
                                    bind:value={formValues.passkeyName}
                                    bind:error={formErrors.passkeyName}
                                    autocomplete="off"
                                    placeholder={t.mfa.passkeyName}
                                    on:enter={handleRegisterPasskey}
                                    width={inputWidth}
                                    disabled={success}
                            >
                                {t.mfa.passkeyName}
                            </Input>
                            <Button
                                    on:click={handleRegisterPasskey} width={btnWidth}
                                    level={success ? 2 : 1}
                                    isDisabled={success}
                            >
                                {t.mfa.register}
                            </Button>

                            {#if success}
                                <div class="success">
                                    <p>{t.passwordReset.successPasskey1}</p>
                                    <p>{t.passwordReset.successPasskey2}</p>
                                    <Button on:click={navigateToAccount} width={btnWidth} level={1}>
                                        {t.passwordReset.accountLogin}
                                    </Button>
                                </div>
                            {/if}
                        </div>
                    {/if}
                {:else if requestType.startsWith('password_reset')}
                    {#if webauthnData}
                        <WebauthnRequest
                                bind:data={webauthnData}
                                purpose="PasswordReset"
                                onSuccess={onWebauthnSuccess}
                                onError={onWebauthnError}
                        />
                    {/if}

                    <h1>Password Reset</h1>

                    <PasswordPolicy bind:accepted policy={data.password_policy} password={formValues.password}/>

                    <PasswordInput
                            bind:value={formValues.password}
                            error={formErrors.password}
                            autocomplete="new-password"
                            placeholder={t.passwordReset.password}
                            width={inputWidth}
                            {showCopy}
                    >
                        {t.passwordReset.password.toUpperCase()}
                    </PasswordInput>
                    <PasswordInput
                            bind:value={formValues.passwordConfirm}
                            error={formErrors.passwordConfirm}
                            autocomplete="new-password"
                            placeholder={t.passwordReset.passwordConfirm}
                            width={inputWidth}
                            {showCopy}
                    >
                        {t.passwordReset.passwordConfirm.toUpperCase()}
                    </PasswordInput>

                    <Button on:click={generate} width={btnWidth} level={3}>
                        {t.passwordReset.generate}
                    </Button>
                    <Button on:click={passwordReset} width={btnWidth} bind:isLoading level={2}>
                        {t.common.save}
                    </Button>

                    {#if success}
                        <div class="success">
                            {t.passwordReset.success1}
                            {t.passwordReset.success2}
                            <br>
                            {t.passwordReset.success3}
                            <br>
                            <a href={redirectUri || '/auth/v1/account'}>Link</a>
                        </div>
                    {/if}
                {/if}

                {#if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}
            </div>
        {/if}
    </ContentCenter>
</Main>

<LangSelector absolute/>

<style>
    a {
        color: var(--col-act2);
    }

    a:visited {
        color: var(--col-act2);
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
