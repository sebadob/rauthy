<script>
    import {onMount} from "svelte";
    import {getSessionInfo} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import {extractFormErrors, getQueryParams, redirectToLogin} from "../../utils/helpers.js";
    import BrowserCheck from "../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import Button from "$lib/Button.svelte";
    import * as yup from "yup";
    import {REGEX_URI} from "../../utils/constants.js";

    /** @type {any} */
    let t;
    /** @type {any} */
    let sessionInfo;

    let err = '';
    let clientId = '';
    let userCodeLength = 8;
    let isLoading = false;
    let onInputValidate = false;

    let formValues = {userCode: ''};
    let formErrors = {userCode: ''};
    let schema = {};
    $: if (t && userCodeLength) {
        schema = yup.object().shape({
            // REGEX_URI is not really correct, but it's not too important either.
            // The backend will validate immediately by cache key, which can be any String.
            userCode: yup.string().trim()
                .min(userCodeLength, t.errTooShort)
                .max(userCodeLength, t.errTooLong)
                .matches(REGEX_URI, t.invalidInput)
        });
    }

    onMount(() => {
        const data = '8\nrauthy'.split('\n');
        // const data = window.document.getElementsByName('rauthy-data')[0].id.split('\n');
        userCodeLength = Number.parseInt(data[0]);
        clientId = data[1];

        const params = getQueryParams();
        if (params.code) {
            formValues.userCode = params.code;
        }
    })

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            sessionInfo = await res.json();
        } else {
            redirectToLogin('device');
        }
    });

    function onInput() {
        if (onInputValidate) {
            validateForm();
        }
    }

    async function onSubmit() {
        err = '';
        onInputValidate = true;

        const valid = await validateForm();
        if (!valid) {
            return;
        }
        isLoading = true;

        console.error('TODO onSubmit');

        isLoading = false;
    }

    async function validateForm() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {userCode: ''};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }
</script>

<svelte:head>
    <title>{t?.title || 'Device Authorization'}</title>
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="device">
        {#if !sessionInfo}
            <Loading/>
        {:else}
            <div class="container">
                <div class="head">
                    <div class="logo">
                        {#if clientId}
                            <img src="{`/auth/v1/clients/${clientId}/logo`}" alt="No Logo Available"/>
                        {/if}
                    </div>
                </div>

                <div class="name">
                    <h2>{t.title}</h2>
                </div>

                <div class="desc">
                    {t.desc.replaceAll('{{count}}', userCodeLength)}
                </div>

                <Input
                        name="userCode"
                        bind:value={formValues.userCode}
                        bind:error={formErrors.userCode}
                        autocomplete="off"
                        placeholder={t.userCode}
                        on:enter={onSubmit}
                        on:input={onInput}
                >
                    {t.userCode.toUpperCase()}
                </Input>

                <Button on:click={onSubmit} bind:isLoading>
                    {t.submit.toUpperCase()}
                </Button>

                <div class="err">{err}</div>
            </div>
        {/if}

        <LangSelector absolute/>
    </WithI18n>
</BrowserCheck>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        max-width: 19rem;
        padding: 20px;
        border: 1px solid var(--col-gmid);
        border-radius: 5px;
        box-shadow: 5px 5px 5px rgba(128, 128, 128, .1);
    }

    .err, .desc {
        margin: 0 .33rem 1rem .33rem;
    }

    .err {
        color: var(--col-err);
    }

    .head {
        display: flex;
        justify-content: space-between;
    }

    .logo {
        margin: 0 .25rem;
        width: 84px;
        height: 84px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .name {
        margin: -10px 5px 0 5px;
    }
</style>
