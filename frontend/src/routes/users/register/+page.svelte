<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { IS_DEV, TPL_RESTRICTED_EMAIL_DOMAIN, TPL_USER_VALUES_CONFIG } from '$utils/constants';
    import Input from '$lib5/form/Input.svelte';
    import LangSelector from '$lib5/LangSelector.svelte';
    import Main from '$lib5/Main.svelte';
    import ContentCenter from '$lib5/ContentCenter.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Template from '$lib5/Template.svelte';
    import { useParam } from '$state/param.svelte';
    import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
    import Form from '$lib5/form/Form.svelte';
    import {
        PATTERN_ALNUM,
        PATTERN_CITY,
        PATTERN_PHONE,
        PATTERN_STREET,
        PATTERN_USER_NAME,
    } from '$utils/patterns';
    import type { NewUserRegistrationRequest } from '$api/types/register.ts';
    import { fetchGet, fetchPost } from '$api/fetch';
    import type { ToSLatestResponse } from '$api/types/tos';
    import { fetchSolvePow } from '$utils/pow';
    import TosAccept from '$lib/TosAccept.svelte';
    import type { UserValuesConfig } from '$api/templates/UserValuesConfig';
    import InputDateTimeCombo from '$lib/form/InputDateTimeCombo.svelte';
    import TZSelect from '$lib/TZSelect.svelte';
    import type { UserValuesRequest } from '$api/types/user';

    let t = useI18n();

    let refEmail: undefined | HTMLInputElement = $state();

    let restrictedDomain = $state('');
    let config: undefined | UserValuesConfig = $state();
    let redirectUri = useParam('redirect_uri');
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    let tos: undefined | ToSLatestResponse = $state();
    let noTosExists = $state(false);
    let usernameExists = $state(false);

    let values: NewUserRegistrationRequest = $state({
        email: '',
        pow: '',
    });
    let valuesActive = $state(1);

    let action = $derived(IS_DEV ? '/auth/v1/dev/register' : '/auth/v1/users/register');

    $effect(() => {
        refEmail?.focus();
    });

    $effect(() => {
        if (config) {
            let active = 1;

            if (config.preferred_username.preferred_username === 'required') {
                values.preferred_username = '';
                active += 1;
            }
            if (config.given_name === 'required') {
                values.given_name = '';
                active += 1;
            }
            if (config.family_name === 'required') {
                values.family_name = '';
                active += 1;
            }

            let uv: UserValuesRequest = {};
            if (config.birthdate === 'required') {
                uv.birthdate = '';
                active += 1;
            }
            if (config.tz !== 'hidden') {
                uv.tz = 'UTC';
                active += 1;
            }
            if (config.street === 'required') {
                uv.street = '';
                active += 1;
            }
            if (config.zip === 'required') {
                uv.zip = '';
                active += 1;
            }
            if (config.city === 'required') {
                uv.city = '';
                active += 1;
            }
            if (config.country === 'required') {
                uv.country = '';
                active += 1;
            }
            if (config.phone === 'required') {
                uv.phone = '';
                active += 1;
            }

            for (let v of Object.keys(uv)) {
                if (v !== undefined) {
                    values.user_values = uv;
                    break;
                }
            }

            valuesActive = active;
        }
    });

    async function fetchTos() {
        tos = undefined;
        noTosExists = false;

        let res = await fetchGet<ToSLatestResponse>('/auth/v1/tos/latest');
        if (res.body) {
            tos = res.body;
        } else if (res.status === 204) {
            noTosExists = true;
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        success = false;
        err = '';

        if (!values.email) {
            return;
        }

        if (restrictedDomain && !values.email.endsWith(restrictedDomain)) {
            err = t.register.domainErr;
            return;
        }

        await fetchTos();

        if (tos) {
            // noop
        } else if (noTosExists) {
            await submitRegistration();
        } else {
            console.error('logic error in ToS fetch / accept');
        }
    }

    async function submitRegistration() {
        isLoading = true;

        values.pow = (await fetchSolvePow()) || '';
        values.redirect_uri = redirectUri.get();

        const res = await fetchPost(action, values);
        if (res.error) {
            let error = res.error.message || 'Error';
            if (error.includes('UNIQUE constraint')) {
                if (error.includes('email')) {
                    err = t.register.alreadyRegisteredEmail;
                } else {
                    err = t.register.alreadyRegisteredUsername;
                }
            } else {
                err = error;
            }
        } else {
            err = '';
            success = true;
            if (redirectUri) {
                setTimeout(() => {
                    window.location.replace(values.redirect_uri || '/auth/v1/account');
                }, 3000);
            }
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t?.register.register || 'Register'}</title>
</svelte:head>

<Template id={TPL_RESTRICTED_EMAIL_DOMAIN} bind:value={restrictedDomain} />
<Template id={TPL_USER_VALUES_CONFIG} bind:value={config} />

<Main>
    <ContentCenter>
        <div class="container">
            <div class="domainTxt">
                <h1>{t.register.userReg}</h1>
                {#if restrictedDomain}
                    {t.register.domainRestricted}<br />
                    {t.register.domainAllowed} <code>@{restrictedDomain}</code>
                {/if}
            </div>

            <Form {action} {onSubmit}>
                <div class={valuesActive > 5 ? 'valuesMany' : ''}>
                    <Input
                        bind:ref={refEmail}
                        typ="email"
                        name="email"
                        autocomplete="email"
                        label={t.common.email}
                        placeholder={t.common.email}
                        bind:value={values.email}
                        required
                    />
                    {#if values.preferred_username !== undefined}
                        <Input
                            name="preferred_username"
                            autocomplete="off"
                            label={t.account.preferredUsername.preferredUsername}
                            placeholder={t.account.preferredUsername.preferredUsername}
                            bind:value={values.preferred_username}
                            required
                            isError={usernameExists}
                            errMsg={usernameExists ? t.account.preferredUsername.notAvailable : ''}
                            pattern={config?.preferred_username.pattern_html}
                            onInput={() => (usernameExists = false)}
                        />
                    {/if}
                    {#if values.given_name !== undefined}
                        <Input
                            name="given_name"
                            autocomplete="given-name"
                            label={t.account.givenName}
                            placeholder={t.account.givenName}
                            bind:value={values.given_name}
                            pattern={PATTERN_USER_NAME}
                            required
                        />
                    {/if}
                    {#if values.family_name !== undefined}
                        <Input
                            name="family_name"
                            autocomplete="family-name"
                            label={t.account.familyName}
                            placeholder={t.account.familyName}
                            bind:value={values.family_name}
                            pattern={PATTERN_USER_NAME}
                            required
                        />
                    {/if}
                    {#if values.user_values?.birthdate !== undefined}
                        <InputDateTimeCombo
                            name="birthdate"
                            label={t.account.birthdate}
                            bind:value={values.user_values.birthdate}
                            required
                            withDelete
                        />
                    {/if}
                    {#if values.user_values?.tz !== undefined && values.user_values?.tz != null}
                        <TZSelect bind:value={values.user_values.tz} />
                    {/if}

                    {#if values.user_values?.street !== undefined}
                        <Input
                            name="street"
                            autocomplete="street-address"
                            label={t.account.street}
                            placeholder={t.account.street}
                            bind:value={values.user_values.street}
                            required
                            maxLength={48}
                            pattern={PATTERN_STREET}
                        />
                    {/if}
                    {#if values.user_values?.zip !== undefined}
                        <Input
                            name="zip"
                            autocomplete="postal-code"
                            label={t.account.zip}
                            placeholder={t.account.zip}
                            bind:value={values.user_values.zip}
                            required
                            maxLength={24}
                            pattern={PATTERN_ALNUM}
                        />
                    {/if}
                    {#if values.user_values?.city !== undefined}
                        <Input
                            name="city"
                            autocomplete="address-level2"
                            label={t.account.city}
                            placeholder={t.account.city}
                            bind:value={values.user_values.city}
                            required
                            maxLength={48}
                            pattern={PATTERN_CITY}
                        />
                    {/if}
                    {#if values.user_values?.country !== undefined}
                        <Input
                            name="country"
                            autocomplete="country"
                            label={t.account.country}
                            placeholder={t.account.country}
                            bind:value={values.user_values.country}
                            required
                            maxLength={48}
                            pattern={PATTERN_CITY}
                        />
                    {/if}
                    {#if values.user_values?.phone !== undefined}
                        <Input
                            name="phone"
                            autocomplete="tel"
                            label={t.account.phone}
                            placeholder={t.account.phone}
                            bind:value={values.user_values.phone}
                            required
                            maxLength={32}
                            pattern={PATTERN_PHONE}
                        />
                    {/if}
                </div>

                <div class="submit">
                    <Button type="submit" {isLoading}>{t.register.register}</Button>
                </div>
                {#if success}
                    <div class="success">
                        {t.register.success}<br />
                        {t.register.emailCheck}
                    </div>
                {:else if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}
            </Form>
        </div>

        {#if tos}
            <TosAccept
                {tos}
                forceAccept
                onToSAccept={submitRegistration}
                onToSCancel={() => (tos = undefined)}
                skipRequest
            />
        {/if}

        <ThemeSwitch absolute />
        <LangSelector absolute />
    </ContentCenter>
</Main>

<style>
    .submit {
        margin-top: 0.66rem;
    }

    .container {
        padding: 1rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / 0.25);
    }

    .err {
        max-width: 16rem;
    }

    .err,
    .success {
        margin-top: 0.5rem;
    }

    .domainTxt {
        margin: 0.5rem 0;
    }

    @media (min-width: 35rem) {
        .valuesMany {
            display: grid;
            grid-template-columns: 1fr 1fr;
            column-gap: 1.5rem;
        }
    }
</style>
