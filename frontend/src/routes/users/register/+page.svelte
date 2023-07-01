<script>
    import * as yup from "yup";
    import {computePow, extractFormErrors} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {REGEX_NAME} from "../../../utils/constants.js";
    import {getPow, registerUser} from "../../../utils/dataFetching.js";
    import {onMount, tick} from "svelte";
    import Input from "$lib/inputs/Input.svelte";
    import BrowserCheck from "../../../components/BrowserCheck.svelte";

    let restrictedDomain;
    let isLoading = false;
    let err = '';
    let success = false;

    let formValues = { email: '', givenName: '', familyName: '' };
    let formErrors = {};

    const schema = yup.object().shape({
        email: yup.string().required('E-Mail is required').email("Bad E-Mail format"),
        givenName: yup.string()
            .required('Given Name is required')
            .matches(REGEX_NAME, "Your given name with 2 - 32  non-special characters"),
        familyName: yup.string()
            .required('Family Name is required')
            .matches(REGEX_NAME, "Your family name with 2 - 32  non-special characters"),
    });

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
            err = 'E-Mail domain not allowed';
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
    <title>Register</title>
</svelte:head>

<BrowserCheck>
    <div class="container">
        {#if restrictedDomain}
            <div class="domainTxt">
                <h1>User Registration</h1>
                E-Mail domains are restricted.<br>
                Allowed domain: <code>@{restrictedDomain}</code>
            </div>
        {/if}

        <Input
                type="email"
                bind:value={formValues.email}
                bind:error={formErrors.email}
                autocomplete="email"
                placeholder="E-Mail"
                on:keypress={handleKeyPress}
        >
            E-MAIL
        </Input>
        <Input
                bind:value={formValues.givenName}
                bind:error={formErrors.givenName}
                autocomplete="given-name"
                placeholder="Given NAme"
                on:keypress={handleKeyPress}
        >
            GIVEN NAME
        </Input>
        <Input
                bind:value={formValues.familyName}
                bind:error={formErrors.familyName}
                autocomplete="family-name"
                placeholder="Family Name"
                on:keypress={handleKeyPress}
        >
            FAMILY NAME
        </Input>

        <Button on:click={onSubmit} bind:isLoading>REGISTER</Button>

        {#if success}
            <div class="success">
                Registration successful.<br/>
                Please check your E-Mail inbox.
            </div>
        {:else if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </div>
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
