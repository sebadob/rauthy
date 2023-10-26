<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../utils/helpers.js";
    import {REGEX_NAME} from "../../utils/constants.js";
    import Button from "$lib/Button.svelte";
    import {blur, fade} from 'svelte/transition';
    import {tweened} from 'svelte/motion';
    import AccModPwd from "./AccModPwd.svelte";
    import {putUserSelf} from "../../utils/dataFetching.js";
    import Input from "$lib/inputs/Input.svelte";

    export let t;
    export let user = {};
    export let viewModePhone = false;
    $: inputWidth = viewModePhone ? 'calc(100vw - 1.5rem)' : '300px';

    const btnWidth = "12rem";

    let isLoading = false;
    let err = '';
    let success = false;
    let successEmailConfirm = false;

    let formValues = {
        email: user.email,
        givenName: user.given_name,
        familyName: user.family_name,
    };
    let formErrors = {};

    const schema = yup.object().shape({
        email: yup.string()
            .required(t.validEmail)
            .email(t.validEmail),
        givenName: yup.string()
            .required(t.validGivenName)
            .matches(REGEX_NAME, t.validGivenName),
        familyName: yup.string()
            .required(t.validFamilyName)
            .matches(REGEX_NAME, t.validFamilyName),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            err = t.invalidInput;
            return;
        }

        isLoading = true;

        const data = {
            email: formValues.email,
            given_name: formValues.givenName,
            family_name: formValues.familyName,
        };

        let res = await putUserSelf(user.id, data);
        if (res.ok) {
            success = true;
            user.email = formValues.email;
            user.given_name = formValues.givenName;
            user.family_name = formValues.familyName;

            if (res.status === 202) {
                successEmailConfirm = true;
            }
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    async function validateForm() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

</script>

<div class="wrapper">
    <div class="container">
        <Input
                bind:value={formValues.email}
                bind:error={formErrors.email}
                autocomplete="family-name"
                placeholder={t.email}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.email.toUpperCase()}
        </Input>
        <Input
                bind:value={formValues.givenName}
                bind:error={formErrors.givenName}
                autocomplete="given-name"
                placeholder={t.givenName}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.givenName.toUpperCase()}
        </Input>
        <Input
                bind:value={formValues.familyName}
                bind:error={formErrors.familyName}
                autocomplete="family-name"
                placeholder={t.familyName}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.familyName.toUpperCase()}
        </Input>

        <Button width={btnWidth} on:click={onSubmit} level={1} bind:isLoading>
            {t.save.toUpperCase()}
        </Button>

        <div class="bottom">
            {#if success}
                <div class="success" transition:fade>
                    Update successful
                    {#if successEmailConfirm}
                        <p>{t.emailUpdateConfirm}</p>
                    {/if}
                </div>
            {:else if err}
                <div class="err" transition:fade>
                    {err}
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .container {
        padding: 0 5px;
        display: flex;
        flex-direction: column;
    }

    .err {
        width: 230px;
        margin: -5px 10px 0 35px;
        padding-right: 5px;
        color: var(--col-err);
    }

    .err {
        margin: 5px;
        color: var(--col-err);
    }

    .bottom {
        height: 1em;
    }

    .success {
        margin: 5px;
        color: var(--col-ok);
    }

    .wrapper {
        margin-top: 10px;
        display: flex;
        flex-direction: row;
    }
</style>
