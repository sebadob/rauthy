<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../utils/helpers";
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
    import {useI18n} from "$state/i18n.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [user]
     * @property {boolean} [viewModePhone]
     */

    /** @type {Props} */
    let {user = $bindable({}), viewModePhone = false} = $props();

    let t = useI18n();

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
        email: yup.string().required(t.account.validEmail).email(t.account.validEmail),
        givenName: yup.string().required(t.account.validGivenName).matches(REGEX_NAME, t.account.validGivenName),
        familyName: yup.string().nullable().matches(REGEX_NAME_NULLABLE, t.account.validFamilyName),
    });

    let formErrorsValues = $state({});
    const schemaValues = yup.object().shape({
        // TODO translations
        birthdate: yup.string().nullable().trim().matches(REGEX_BIRTHDATE, t.common.invalidInput),
        phone: yup.string().nullable().trim().matches(REGEX_PHONE, '+...'),
        street: yup.string().nullable().trim().matches(REGEX_STREET, t.common.invalidInput),
        zip: yup.number().nullable().min(1000).max(999999),
        city: yup.string().nullable().trim().matches(REGEX_CITY, t.common.invalidInput),
        country: yup.string().nullable().trim().matches(REGEX_CITY, t.common.invalidInput),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            err = t.common.invalidInput;
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
                placeholder={t.common.email}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.common.email.toUpperCase()}
        </Input>
        <Input
                bind:value={formValues.givenName}
                bind:error={formErrors.givenName}
                autocomplete="given-name"
                placeholder={t.account.givenName}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.givenName.toUpperCase()}
        </Input>
        <Input
                bind:value={formValues.familyName}
                bind:error={formErrors.familyName}
                autocomplete="family-name"
                placeholder={t.account.familyName}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.familyName.toUpperCase()}
        </Input>

        <div style:margin=".5rem">
            {t.account.optionalValues}
        </div>

        <!-- Street-->
        <Input
                bind:value={user.user_values.street}
                bind:error={formErrorsValues.street}
                autocomplete="street-address"
                placeholder={t.account.street}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.street.toUpperCase()}
        </Input>

        <!-- ZIP-->
        <Input
                type="number"
                bind:value={user.user_values.zip}
                bind:error={formErrorsValues.zip}
                autocomplete="postal-code"
                placeholder={t.account.zip}
                min={1000}
                max={999999}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.zip.toUpperCase()}
        </Input>

        <!-- City-->
        <Input
                bind:value={user.user_values.city}
                bind:error={formErrorsValues.city}
                autocomplete="adress-level2"
                placeholder={t.account.city}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.city.toUpperCase()}
        </Input>

        <!-- Country-->
        <Input
                bind:value={user.user_values.country}
                bind:error={formErrorsValues.country}
                autocomplete="country"
                placeholder={t.account.country}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.country.toUpperCase()}
        </Input>

        <!-- Phone-->
        <Input
                bind:value={user.user_values.phone}
                bind:error={formErrorsValues.phone}
                autocomplete="tel"
                placeholder={t.account.phone}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.phone.toUpperCase()}
        </Input>

        <!-- Birthdate-->
        <Input
                type="date"
                bind:value={user.user_values.birthdate}
                bind:error={formErrorsValues.birthdate}
                autocomplete="bday"
                placeholder={t.account.birthdate}
                on:keypress={handleKeyPress}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.account.birthdate.toUpperCase()}
        </Input>

        <Button width={btnWidth} on:click={onSubmit} level={1} bind:isLoading>
            {t.common.save}
        </Button>

        <div class="bottom">
            {#if success}
                <div class="success" transition:fade>
                    Update successful
                    {#if successEmailConfirm}
                        <p>{t.account.emailUpdateConfirm}</p>
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
