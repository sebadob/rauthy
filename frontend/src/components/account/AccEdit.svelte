<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../utils/helpers.js";
    import {
        REGEX_BIRTHDATE,
        REGEX_CITY,
        REGEX_NAME,
        REGEX_NAME_NULLABLE,
        REGEX_PHONE,
        REGEX_STREET
    } from "../../utils/constants.js";
    import Button from "$lib/Button.svelte";
    import {fade} from 'svelte/transition';
    import {putUserSelf} from "../../utils/dataFetching.js";
    import Input from "$lib/inputs/Input.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} t
     * @property {any} [user]
     * @property {boolean} [viewModePhone]
     */

    /** @type {Props} */
    let {t, user = $bindable({}), viewModePhone = false} = $props();
    let inputWidth = $derived(viewModePhone ? 'calc(100vw - 1.5rem)' : '300px');

    const btnWidth = "8rem";

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let successEmailConfirm = $state(false);

    let formValues = $state({
        email: user.email,
        givenName: user.given_name,
        familyName: user.family_name,
    });
    let formErrors = $state({});

    const schema = yup.object().shape({
        email: yup.string().required(t.validEmail).email(t.validEmail),
        givenName: yup.string().required(t.validGivenName).matches(REGEX_NAME, t.validGivenName),
        familyName: yup.string().nullable().matches(REGEX_NAME_NULLABLE, t.validFamilyName),
    });

    let formErrorsValues = $state({});
    const schemaValues = yup.object().shape({
        // TODO translations
        birthdate: yup.string().nullable().trim().matches(REGEX_BIRTHDATE, t.invalidInput),
        phone: yup.string().nullable().trim().matches(REGEX_PHONE, '+...'),
        street: yup.string().nullable().trim().matches(REGEX_STREET, t.invalidInput),
        zip: yup.number().nullable().min(1000).max(999999),
        city: yup.string().nullable().trim().matches(REGEX_CITY, t.invalidInput),
        country: yup.string().nullable().trim().matches(REGEX_CITY, t.invalidInput),
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
            family_name: formValues.familyName || null,
            user_values: user.user_values,
        };

        if (data.user_values.phone) {
            data.user_values.phone = data.user_values.phone.replaceAll(' ', '');
        }
        if (data.user_values.zip) {
            data.user_values.zip = Number.parseInt(data.user_values.zip);
        }

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
        let isOk = true;

        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            isOk = false;
        }

        try {
            await schemaValues.validate(user.user_values, {abortEarly: false});
            formErrorsValues = {};
        } catch (err) {
            formErrorsValues = extractFormErrors(err);
            isOk = false;
        }

        return isOk;
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

        <div style:margin=".5rem">
            {t.optionalValues}
        </div>

        <!-- Street-->
        <Input
                bind:value={user.user_values.street}
                bind:error={formErrorsValues.street}
                autocomplete="street-address"
                placeholder={t.street}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.street.toUpperCase()}
        </Input>

        <!-- ZIP-->
        <Input
                type="number"
                bind:value={user.user_values.zip}
                bind:error={formErrorsValues.zip}
                autocomplete="postal-code"
                placeholder={t.zip}
                min={1000}
                max={999999}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.zip.toUpperCase()}
        </Input>

        <!-- City-->
        <Input
                bind:value={user.user_values.city}
                bind:error={formErrorsValues.city}
                autocomplete="adress-level2"
                placeholder={t.city}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.city.toUpperCase()}
        </Input>

        <!-- Country-->
        <Input
                bind:value={user.user_values.country}
                bind:error={formErrorsValues.country}
                autocomplete="country"
                placeholder={t.country}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.country.toUpperCase()}
        </Input>

        <!-- Phone-->
        <Input
                bind:value={user.user_values.phone}
                bind:error={formErrorsValues.phone}
                autocomplete="tel"
                placeholder={t.phone}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.phone.toUpperCase()}
        </Input>

        <!-- Birthdate-->
        <Input
                type="date"
                bind:value={user.user_values.birthdate}
                bind:error={formErrorsValues.birthdate}
                autocomplete="bday"
                placeholder={t.birthdate}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.birthdate.toUpperCase()}
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
