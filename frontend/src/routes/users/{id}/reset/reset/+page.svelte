<script lang="ts">
    import {generatePassword} from "$utils/helpers";
    import Button from "$lib5/button/Button.svelte";
    import PasswordPolicy from "$lib5/PasswordPolicy.svelte";
    import Input from "$lib5/form/Input.svelte";
    import WebauthnRequest from "$lib5/WebauthnRequest.svelte";
    import {slide} from "svelte/transition";
    import LangSelector from "$lib5/LangSelector.svelte";
    import {IS_DEV, TPL_PASSWORD_RESET} from "$utils/constants";
    import {useI18n} from "$state/i18n.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import type {PasswordResetTemplate} from "$api/templates/PasswordReset";
    import Template from "$lib5/Template.svelte";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import type {MfaPurpose, WebauthnAdditionalData} from "$webauthn/types.ts";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import A from "$lib5/A.svelte";
    import {webauthnReg} from "$webauthn/registration";
    import Form from "$lib5/form/Form.svelte";
    import {PATTERN_USER_NAME} from "$utils/patterns";
    import type {PasswordResetRequest} from "$api/types/password_reset.ts";

    const inputWidth = '20rem';

    let t = useI18n();

    let tplData: undefined | PasswordResetTemplate = $state();

    let refPasskey: undefined | HTMLInputElement = $state();
    let refPassword: undefined | HTMLInputElement = $state();

    let isLoading = $state(false);
    let err = $state('');
    let requestType = useParam('type', 'password_reset');
    let accountTypeNew = $state('');
    let redirectUri = $state('');
    let success = $state(false);
    let accepted = $state(false);

    let mfaPurpose: undefined | MfaPurpose = $state();

    let passkeyName = $state('');
    let password = $state('');
    let passwordConfirm = $state('');

    let reportValidityNew: undefined | (() => void) = $state();
    let reportValidityConfirm: undefined | (() => void) = $state();

    $effect(() => {
        if (accountTypeNew) {
            resetValues();
        }
    });

    $effect(() => {
        refPasskey?.focus();
    });

    $effect(() => {
        refPassword?.focus();
    });

    $effect(() => {
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

    function navigateToAccount() {
        window.location.replace('/auth/v1/account');
    }

    function generate() {
        if (tplData) {
            let pwd = generatePassword(tplData.password_policy);
            password = pwd;
            passwordConfirm = pwd;

            requestAnimationFrame(() => {
                reportValidityNew?.();
                reportValidityConfirm?.();
            });
        }
    }

    async function handleRegister() {
        if (!tplData) {
            console.error('template data is undefined');
            return;
        }

        err = '';

        if (passkeyName.length < 1) {
            err = t.mfa.passkeyNameErr;
            return;
        }

        let res = await webauthnReg(
            tplData.user_id,
            passkeyName,
            t.authorize.invalidKeyUsed,
            t.authorize.requestExpired,
            tplData.magic_link_id,
            tplData.csrf_token,
        );
        if (res.error) {
            err = `${t.mfa.errorReg} - ${res.error}`;
        } else {
            resetValues();
            success = true;
        }
    }

    async function passwordReset() {
        err = '';

        if (!accepted) {
            return;
        }
        if (password !== passwordConfirm) {
            err = t.passwordReset.passwordNoMatch;
            return;
        }
        if (password.length > 256) {
            err = 'max 256';
            return;
        }

        if (tplData?.needs_mfa) {
            mfaPurpose = 'PasswordReset';
        } else {
            await onSubmitFinish();
        }
    }

    async function onSubmitFinish(mfaCode?: string) {
        if (!tplData) {
            return;
        }

        isLoading = true;

        const payload: PasswordResetRequest = {
            password,
            magic_link_id: tplData.magic_link_id,
            mfa_code: mfaCode,
        };
        let res = await fetch(`/auth/v1/users/${tplData.user_id}/reset`, {
            method: 'PUT',
            headers: {
                'content-type': 'application/json',
                // TODO prefix with X-
                // The same should be true for the session token
                // -> mention breaking change in changelog when doing that!
                'x-pwd-csrf-token': tplData?.csrf_token,
            },
            body: JSON.stringify(payload),
        });
        if (res.ok) {
            resetValues();
            redirectUri = res.headers.get('location') || '/auth/v1/account';
            success = true;
        } else {
            const body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    function onWebauthnError(error: string) {
        err = error;
        mfaPurpose = undefined;
    }

    function onWebauthnSuccess(data?: WebauthnAdditionalData) {
        mfaPurpose = undefined;
        if (data && 'code' in data) {
            onSubmitFinish(data.code)
        } else {
            console.error('invalid webauthn response', data);
        }
    }

    function resetValues() {
        err = '';
        passkeyName = '';
        password = '';
        passwordConfirm = '';
    }
</script>

<svelte:head>
    {#if t}
        {#if requestType.get()?.startsWith('new_user')}
            <title>{t.passwordReset.newAccount}</title>
        {:else if requestType.get() === "password_reset"}
            <title>{t.passwordReset.passwordReset}</title>
        {/if}
    {:else}
        <title>Password</title>
    {/if}
</svelte:head>

<Template id={TPL_PASSWORD_RESET} bind:value={tplData}/>

{#snippet passwordInput()}
    {#if tplData}
        <Form action="" onSubmit={passwordReset}>
            <PasswordPolicy
                    bind:accepted
                    policy={tplData.password_policy}
                    password={password}
            />

            <InputPassword
                    bind:ref={refPassword}
                    bind:value={password}
                    autocomplete="new-password"
                    label={t.account.passwordNew}
                    placeholder={t.account.passwordNew}
                    maxLength={tplData.password_policy.length_max}
                    required
                    bind:reportValidity={reportValidityNew}
                    showCopy={password.length >= tplData.password_policy.length_min}
                    width={inputWidth}
            />
            <InputPassword
                    bind:value={passwordConfirm}
                    autocomplete="new-password"
                    label={t.account.passwordConfirm}
                    placeholder={t.account.passwordConfirm}
                    maxLength={tplData.password_policy.length_max}
                    bind:reportValidity={reportValidityConfirm}
                    required
                    width={inputWidth}
            />

            <div class="generate">
                <Button level={2} onclick={generate}>
                    {t.passwordReset.generate}
                </Button>
            </div>

            <Button type="submit" {isLoading}>
                {t.common.save}
            </Button>
        </Form>
    {/if}
{/snippet}

{#if IS_DEV}
    <div class="dev">
        <p>
            This window shows up during local dev,<br>
            only to be able to switch modes easily.<br>
        </p>
        <Button level={2} onclick={() => requestType.set('new_user')}>
            new_user
        </Button>
        <Button level={2} onclick={() => requestType.set('password_reset')}>
            password_reset
        </Button>
    </div>
{/if}

<Main>
    <ContentCenter>
        {#if mfaPurpose && tplData}
            <WebauthnRequest
                    userId={tplData.user_id}
                    purpose={mfaPurpose}
                    onSuccess={onWebauthnSuccess}
                    onError={onWebauthnError}
            />
        {/if}

        {#if success}
            <p>
                {t.passwordReset.success1}
                <br>
                {t.passwordReset.success2}
                <br>
                <br>
                {t.passwordReset.success3}
                <A href={redirectUri || '/auth/v1/account'}>Account</A>
            </p>
        {:else if tplData}
            <div class="container">
                {#if requestType.get()?.startsWith('new_user')}
                    <h1>{t.passwordReset.newAccount}</h1>
                    <p>{t.passwordReset.newAccDesc1}</p>
                    <p>{
                        t.passwordReset.newAccDesc2}
                        <A href={t.passwordReset.fidoLink} target="_blank">
                            FIDO Alliance
                        </A>
                    </p>

                    <div class="typeChoice">
                        <Button
                                level={!accountTypeNew ? 1 : 3}
                                onclick={() => accountTypeNew = "passkey"}
                                {isLoading}
                        >
                            {t.passwordReset.passwordless}
                        </Button>
                        <Button
                                level={!accountTypeNew ? 2 : 3}
                                onclick={() => accountTypeNew = "password"}
                                {isLoading}
                        >
                            {t.passwordReset.password}
                        </Button>
                    </div>

                    {#if accountTypeNew === "password"}
                        <div transition:slide>
                            {@render passwordInput()}
                        </div>
                    {:else if accountTypeNew === "passkey"}
                        <div transition:slide>
                            <Form action="" onSubmit={handleRegister}>
                                <Input
                                        bind:ref={refPasskey}
                                        bind:value={passkeyName}
                                        autocomplete="off"
                                        label={t.mfa.passkeyName}
                                        placeholder={t.mfa.passkeyName}
                                        width={inputWidth}
                                        maxLength={32}
                                        pattern={PATTERN_USER_NAME}
                                        required
                                />
                                <Button
                                        type="submit"
                                        level={success ? 2 : 1}
                                >
                                    {t.mfa.register}
                                </Button>
                            </Form>

                            {#if success}
                                <div class="success">
                                    <p>{t.passwordReset.successPasskey1}</p>
                                    <p>{t.passwordReset.successPasskey2}</p>
                                    <Button onclick={navigateToAccount}>
                                        {t.passwordReset.accountLogin}
                                    </Button>
                                </div>
                            {/if}
                        </div>
                    {/if}
                {:else if requestType.get()?.startsWith('password_reset')}
                    <h1>Password Reset</h1>
                    {@render passwordInput()}
                {/if}

                {#if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}
            </div>
        {/if}

        <ThemeSwitch absolute/>
    </ContentCenter>
</Main>

<LangSelector absolute/>

<style>
    .err {
        margin-top: .5rem;
        max-width: 20rem;
        color: hsl(var(--error));
    }

    .dev {
        position: absolute;
        top: .5rem;
        right: .5rem;
        padding: .5rem;
        border-radius: var(--border-radius);
        background: hsl(var(--bg-high));
    }

    .generate {
        margin-bottom: .66rem;
    }

    .typeChoice {
        margin-bottom: 1rem;
        display: flex;
        gap: .5rem;
    }
</style>
