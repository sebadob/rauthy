<script>
    import * as yup from "yup";
    import {computePow, extractFormErrors} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {REGEX_NAME} from "../../../utils/constants.js";
    import {getPow, registerUser} from "../../../utils/dataFetching.js";
    import {onMount, tick} from "svelte";
    import Input from "$lib/inputs/Input.svelte";
    import BrowserCheck from "../../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";

    let t;
    let restrictedDomain;
    let isLoading = false;
    let err = '';
    let success = false;

    let formValues = { email: '', givenName: '', familyName: '' };
    let formErrors = {};

    let schema = {};
    $: if (t) {
        schema = yup.object().shape({
            email: yup.string().required(t.required).email(t.emailBadFormat),
            givenName: yup.string()
                .required(t.required)
                .matches(REGEX_NAME, t.regexName),
            familyName: yup.string()
                .required(t.required)
                .matches(REGEX_NAME, t.regexName),
        });
    }

    onMount(() => {
        restrictedDomain = window.document.getElementsByName('rauthy-data')[0].id;
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

        if (!formValues.email.endsWith(restrictedDomain)) {
            err = t.domainErr;
            return;
        }

        isLoading = true;
        await tick();

        // compute PoW
        const powRes = await getPow();
        let powChallenge = await powRes.json();
        // console.log(powChallenge);
        let start = new Date().getUTCMilliseconds();
        const pow = computePow(powChallenge);
        let diff = new Date().getUTCMilliseconds() - start;
        console.log('pow computation took ' + diff + ' ms');

        // build payload
        const data = {
            email: formValues.email,
            given_name: formValues.givenName,
            family_name: formValues.familyName,
            pow,
        };

        const res = await registerUser(data);
        if (res.ok) {
            err = '';
            success = true;
        } else {
            const body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t?.register || 'Register'}</title>
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="register">
        <div class="container">
            {#if restrictedDomain}
                <div class="domainTxt">
                    <h1>{t.userReg}</h1>
                    {t.domainRestricted}<br>
                    {t.domainAllowed} <code>@{restrictedDomain}</code>
                </div>
            {/if}

            <Input
                    type="email"
                    bind:value={formValues.email}
                    bind:error={formErrors.email}
                    autocomplete="email"
                    placeholder={t.email}
                    on:keypress={handleKeyPress}
            >
                {t.email.toUpperCase()}
            </Input>
            <Input
                    bind:value={formValues.givenName}
                    bind:error={formErrors.givenName}
                    autocomplete="given-name"
                    placeholder={t.givenName}
                    on:keypress={handleKeyPress}
            >
                {t.givenName.toUpperCase()}
            </Input>
            <Input
                    bind:value={formValues.familyName}
                    bind:error={formErrors.familyName}
                    autocomplete="family-name"
                    placeholder={t.familyName}
                    on:keypress={handleKeyPress}
            >
                {t.familyName.toUpperCase()}
            </Input>

            <Button on:click={onSubmit} bind:isLoading>{t.register.toUpperCase()}</Button>

            {#if success}
                <div class="success">
                    {t.success}<br/>
                    {t.emailCheck}
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
