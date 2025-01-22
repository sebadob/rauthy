<script>
    import {run} from 'svelte/legacy';
    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers";
    import Button from "$lib/Button.svelte";
    import {REGEX_NAME, REGEX_NAME_NULLABLE, TPL_RESTRICTED_EMAIL_DOMAIN} from "../../../utils/constants.js";
    import {registerUser} from "../../../utils/dataFetching.js";
    import {tick} from "svelte";
    import Input from "$lib/inputs/Input.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import {fetchSolvePow} from "../../../utils/pow.ts";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";

    let t = useI18n();
    let restrictedDomain = $state('');
    let redirectUri = useParam('redirect_uri');
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    let formValues = $state({email: '', givenName: '', familyName: ''});
    let formErrors = $state({});

    let schema = $state({});
    run(() => {
        if (t) {
            schema = yup.object().shape({
                email: yup.string().required(t.common.required).email(t.register.emailBadFormat),
                givenName: yup.string()
                    .required(t.common.required)
                    .matches(REGEX_NAME, t.register.regexName),
                familyName: yup.string()
                    .matches(REGEX_NAME_NULLABLE, t.register.regexName),
            });
        }
    });

    function handleKeyPress(event) {
        if (event.detail.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        success = false;
        err = '';

        // validate form
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        if (restrictedDomain && !formValues.email.endsWith(restrictedDomain)) {
            err = t.register.domainErr;
            return;
        }

        isLoading = true;
        await tick();

        let pow = await fetchSolvePow();
        const data = {
            email: formValues.email,
            given_name: formValues.givenName,
            family_name: formValues.familyName.length > 0 ? formValues.familyName : undefined,
            pow,
        };

        // this allows to redirect the client to a custom URI after a successful password set
        let uri = redirectUri.get();
        if (uri) {
            data.redirect_uri = uri;
        }

        const res = await registerUser(data);
        if (res.ok) {
            err = '';
            success = true;
            if (redirectUri) {
                setTimeout(() => {
                    window.location.replace(redirectUri.get() || '/auth/v1/account');
                }, 3000);
            }
        } else {
            const body = await res.json();
            if (body.message.includes("UNIQUE constraint")) {
                err = 'E-Mail is already registered';
            } else {
                err = body.message;
            }
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t?.register || 'Register'}</title>
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

            <Input
                    type="email"
                    bind:value={formValues.email}
                    bind:error={formErrors.email}
                    autocomplete="email"
                    placeholder={t.common.email}
                    on:keypress={handleKeyPress}
            >
                {t.common.email.toUpperCase()}
            </Input>
            <Input
                    bind:value={formValues.givenName}
                    bind:error={formErrors.givenName}
                    autocomplete="given-name"
                    placeholder={t.account.givenName}
                    on:keypress={handleKeyPress}
            >
                {t.account.givenName.toUpperCase()}
            </Input>
            <Input
                    bind:value={formValues.familyName}
                    bind:error={formErrors.familyName}
                    autocomplete="family-name"
                    placeholder={t.account.familyName}
                    on:keypress={handleKeyPress}
            >
                {t.account.familyName.toUpperCase()}
            </Input>

            <Button on:click={onSubmit} bind:isLoading>{t.register.register}</Button>

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
        </div>

        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .domainTxt {
        margin: 0 5px 15px 5px;
    }

    .err {
        margin: 0 5px;
        color: var(--col-err);
    }

    .success {
        margin: 0 5px;
    }
</style>
