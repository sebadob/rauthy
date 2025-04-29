<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {TPL_RESTRICTED_EMAIL_DOMAIN} from "$utils/constants";
    import Input from "$lib5/form/Input.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import Form from "$lib5/form/Form.svelte";
    import {useIsDev} from "$state/is_dev.svelte";
    import {PATTERN_USER_NAME} from "$utils/patterns";
    import type {NewUserRegistrationRequest} from "$api/types/register.ts";
    import {fetchPost} from "$api/fetch";

    let t = useI18n();
    let isDev = useIsDev();

    let restrictedDomain = $state('');
    let redirectUri = useParam('redirect_uri');
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    let action = $derived(isDev.get() ? '/auth/v1/dev/register' : '/auth/v1/users/register');

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        success = false;
        err = '';

        let email = params.get('email');
        let given_name = params.get('given_name');
        let pow = params.get('pow');

        if (!email || !given_name || !pow) {
            console.error('email, given_name, pow missing');
            return;
        }

        if (restrictedDomain && !email.endsWith(restrictedDomain)) {
            err = t.register.domainErr;
            return;
        }

        let payload: NewUserRegistrationRequest = {
            email,
            given_name,
            family_name: params.get('family_name') || undefined,
            pow,
            redirect_uri: params.get('redirect_uri') || undefined,
        };

        isLoading = true;
        // await tick();

        const res = await fetchPost(form.action, payload);
        if (res.error) {
            let error = res.error.message || 'Error';
            if (error.includes("UNIQUE constraint")) {
                err = 'E-Mail is already registered';
            } else {
                err = error;
            }
        } else {
            err = '';
            success = true;
            if (redirectUri) {
                setTimeout(() => {
                    window.location.replace(payload.redirect_uri || '/auth/v1/account');
                }, 3000);
            }
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t?.register.register || 'Register'}</title>
</svelte:head>

<Template id={TPL_RESTRICTED_EMAIL_DOMAIN} bind:value={restrictedDomain}/>

<Main>
    <ContentCenter>
        <div class="container">
            <div class="domainTxt">
                <h1>{t.register.userReg}</h1>
                {#if restrictedDomain}
                    {t.register.domainRestricted}<br>
                    {t.register.domainAllowed} <code>@{restrictedDomain}</code>
                {/if}
            </div>

            <Form {action} {onSubmit} withPowAs="pow">
                {#if redirectUri.get()}
                    <input type="hidden" name="redirect_uri" value={redirectUri.get()}/>
                {/if}
                <Input
                        typ="email"
                        name="email"
                        autocomplete="email"
                        label={t.common.email}
                        placeholder={t.common.email}
                        required
                />
                <Input
                        name="given_name"
                        autocomplete="given-name"
                        label={t.account.givenName}
                        placeholder={t.account.givenName}
                        pattern={PATTERN_USER_NAME}
                        required
                />
                <Input
                        name="family_name"
                        autocomplete="family-name"
                        label={t.account.familyName}
                        placeholder={t.account.familyName}
                        pattern={PATTERN_USER_NAME}
                />

                <div class="submit">
                    <Button type="submit" {isLoading}>{t.register.register}</Button>
                </div>
                {#if success}
                    <div class="success">
                        {t.register.success}<br/>
                        {t.register.emailCheck}
                    </div>
                {:else if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}

            </Form>
        </div>

        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>

<style>
    .submit {
        margin-top: .66rem;
    }

    .container {
        padding: 1rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / .25);
    }

    .err {
        max-width: 16rem;
    }

    .err, .success {
        margin-top: .5rem;
    }

    .domainTxt {
        margin: .5rem 0;
    }
</style>
