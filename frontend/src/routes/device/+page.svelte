<script>
    import {run} from 'svelte/legacy';
    import {onMount} from "svelte";
    import {postDeviceVerify, getPow, getSessionInfo} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import {extractFormErrors, getQueryParams, redirectToLogin} from "../../utils/helpers.js";
    import BrowserCheck from "../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import Button from "$lib/Button.svelte";
    import * as yup from "yup";
    import {REGEX_URI} from "../../utils/constants.js";
    import {fetchSolvePow} from "../../utils/pow.ts";

    const btnWidthInline = '8rem';

    /** @type {any} */
    let t = $state();
    /** @type {any} */
    let sessionInfo = $state();

    let err = $state('');
    let userCodeLength = $state(8);
    let isLoading = $state(false);
    let onInputValidate = false;

    /** @type {string | undefined} */
    let scopes = $state(undefined);
    let isAccepted = $state(false);
    let isDeclined = $state(false);

    let formValues = $state({userCode: ''});
    let formErrors = $state({userCode: ''});
    let schema = $state({});
    run(() => {
        if (t && userCodeLength) {
            schema = yup.object().shape({
                // REGEX_URI is not really correct, but it's not too important either.
                // The backend will validate immediately by cache key, which can be any String.
                userCode: yup.string().trim()
                    .min(userCodeLength, t.errTooShort)
                    .max(userCodeLength, t.errTooLong)
                    .matches(REGEX_URI, t.invalidInput)
            });
        }
    });

    onMount(() => {
        userCodeLength = Number.parseInt(window.document.getElementsByName('rauthy-data')[0].id);
    })

    onMount(async () => {
        const params = getQueryParams();
        if (params.code) {
            formValues.userCode = params.code;
        }

        let res = await getSessionInfo();
        if (res.ok) {
            sessionInfo = await res.json();
        } else if (params.code) {
            redirectToLogin(`device?code=${params.code}`);
        } else {
            redirectToLogin('device');
        }
    });

    function onInput() {
        if (onInputValidate) {
            validateForm();
        }
    }

    async function onSubmit(deviceAccepted) {
        err = '';
        onInputValidate = true;

        const valid = await validateForm();
        if (!valid) {
            return;
        }
        isLoading = true;

        let pow = await fetchSolvePow();
        let data = {
            user_code: formValues.userCode,
            pow,
            device_accepted: deviceAccepted,
        };
        const res = await postDeviceVerify(data);
        if (res.status === 200) {
            const body = await res.json();
            console.log(body);
            scopes = body.scopes?.split(' ') || ['openid'];
        } else if (res.status === 202) {
            isAccepted = true;
            setTimeout(() => {
                window.location.replace('/auth/v1/account?v=devices');
            }, 2000);
        } else if (res.status === 204) {
            isDeclined = true;
        } else if (res.status === 404) {
            err = t.wrongOrExpired;
        } else {
            const body = await res.json();
            err = body.message;
        }

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
                <div class="name">
                    <h2>{t.title}</h2>
                </div>

                {#if scopes === undefined}
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

                    <Button on:click={() => onSubmit('pending')} bind:isLoading>
                        {t.submit.toUpperCase()}
                    </Button>
                {:else if isAccepted}
                    <div class="desc">
                        <p>{t.isAccepted}</p>
                        <p>{t.autoRedirectAccount}</p>
                    </div>
                {:else if isDeclined}
                    <div class="desc">
                        <p class="declined">{t.isDeclined}</p>
                        <p>{t.closeWindow}</p>
                    </div>
                {:else}
                    <div class="desc">
                        {t.descScopes}
                        <ul>
                            {#each scopes as scope}
                                <li>{scope}</li>
                            {/each}
                        </ul>
                    </div>

                    <div class="inline">
                        <Button
                                on:click={() => onSubmit('accept')}
                                bind:isLoading
                                level={1}
                                width={btnWidthInline}
                        >
                            {t.accept}
                        </Button>
                        <Button
                                on:click={() => onSubmit('decline')}
                                bind:isLoading
                                level={3}
                                width={btnWidthInline}
                        >
                            {t.decline}
                        </Button>
                    </div>
                {/if}

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

    .declined {
        color: var(--col-err);
    }

    .err, .desc {
        margin: 0 .33rem 1rem .33rem;
    }

    .err {
        color: var(--col-err);
    }

    .inline {
        display: flex;
        justify-content: space-between;
    }

    .name {
        margin: -10px 5px 0 5px;
    }
</style>
