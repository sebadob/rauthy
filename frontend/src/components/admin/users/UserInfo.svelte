<script>
    import * as yup from "yup";
    import {
        extractFormErrors,
        formatDateFromTs,
        formatDateToDateInput,
        formatUtcTsFromDateInput, redirectToLogin
    } from "../../../utils/helpers.js";
    import Switch from "$lib/Switch.svelte";
    import {globalGroupsNames, globalRolesNames} from "../../../stores/admin.js";
    import Button from "$lib/Button.svelte";
    import {
        LANGUAGES,
        REGEX_BIRTHDATE,
        REGEX_CITY,
        REGEX_NAME,
        REGEX_PHONE,
        REGEX_STREET
    } from "../../../utils/constants.js";
    import {putUser} from "../../../utils/dataFetchingAdmin.js";
    import {onMount} from "svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";

    export let user = {};
    export let onSave;

    let err = '';
    let success = false;
    let timer;
    let language = user.language.toUpperCase();
    let limitLifetime = !!user.user_expires;
    let userExpires = limitLifetime ? formatDateFromTs(user.user_expires, true) : undefined;

    let allRoles = [];
    globalRolesNames.subscribe(rls => {
        allRoles = rls;
    })

    let allGroups = [];
    globalGroupsNames.subscribe(grps => {
        allGroups = grps;
    })

    $: if (success) {
        timer = setTimeout(() => {
            success = false;
            onSave();
        }, 3000);
    }

    let formErrors = {};
    const schema = yup.object().shape({
        email: yup.string().required('E-Mail is required').email("Bad E-Mail format"),
        given_name: yup.string().trim().required('Given Name is required').matches(REGEX_NAME, 'Invalid characters'),
        family_name: yup.string().trim().required('Family Name is required').matches(REGEX_NAME, 'Invalid characters'),
    });

    let formErrorsValues = {};
    const schemaValues = yup.object().shape({
        birthdate: yup.string().nullable().trim().matches(REGEX_BIRTHDATE, 'Invalid characters'),
        phone: yup.string().nullable().trim().matches(REGEX_PHONE, 'Format: +49...'),
        street: yup.string().nullable().trim().matches(REGEX_STREET, 'Invalid characters'),
        zip: yup.number().nullable().min(1000).max(999999),
        city: yup.string().nullable().trim().matches(REGEX_CITY, 'Invalid characters'),
        country: yup.string().nullable().trim().matches(REGEX_CITY, 'Invalid characters'),
    });

    onMount(() => {
        return () => clearTimeout(timer);
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    // TODO update submit
    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            return;
        }
        err = '';

        console.log(user.user_values);
        const req = {
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            language: language.toLowerCase(),
            roles: user.roles,
            groups: user.groups,
            enabled: user.enabled,
            email_verified: user.email_verified,
            user_expires: null,
            user_values: user.user_values,
        };

        if (req.user_values.zip) {
            req.user_values.zip = Number.parseInt(req.user_values.zip);
        }

        if (limitLifetime) {
            let d = formatUtcTsFromDateInput(userExpires);
            if (!d) {
                err = 'Invalid Date Input: User Expires';
                return;
            }
            req.user_expires = d;
        }

        let res = await putUser(user.id, req);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

    async function validateForm() {
        let isOk = true;

        try {
            await schema.validate(user, {abortEarly: false});
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

<div class="container">
    <!-- UID-->
    <div class="unit">
        <div class="label font-label">
            USER ID
        </div>
        <div class="value font-mono">
            {user.id}
        </div>
    </div>

    <div class="row" style:margin-top="-5px">
        <!-- User Enabled -->
        <div class="unit" style:width="107px">
            <div class="label font-label">
                ENABLED
            </div>
            <div class="value">
                <Switch bind:selected={user.enabled}/>
            </div>
        </div>

        <!-- E-Mail verified -->
        <div class="unit" style:width="135px">
            <div class="label font-label">
                E-MAIL VERIFIED
            </div>
            <div class="value">
                <Switch bind:selected={user.email_verified}/>
            </div>
        </div>
    </div>

    <!-- E-Mail-->
    <Input
            type="email"
            bind:value={user.email}
            bind:error={formErrors.email}
            autocomplete="off"
            placeholder="E-Mail"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        E-MAIL
    </Input>

    <!-- Given Name-->
    <Input
            bind:value={user.given_name}
            bind:error={formErrors.given_name}
            autocomplete="off"
            placeholder="Given Name"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        GIVEN NAME
    </Input>

    <!-- Family Name-->
    <Input
            bind:value={user.family_name}
            bind:error={formErrors.family_name}
            autocomplete="off"
            placeholder="Family Name"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        FAMILY NAME
    </Input>

    <!-- Language-->
    <div class="unit language">
        <div class="label">
            LANGUAGE
        </div>
        <div style="margin-top: .085rem">
            <OptionSelect bind:value={language} options={LANGUAGES} />
        </div>
    </div>

    <!-- Street-->
    <Input
            bind:value={user.user_values.street}
            bind:error={formErrorsValues.street}
            autocomplete="off"
            placeholder="Street"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        STREET
    </Input>

    <!-- ZIP-->
    <Input
            type="number"
            bind:value={user.user_values.zip}
            bind:error={formErrorsValues.zip}
            autocomplete="off"
            placeholder="ZIP"
            min={1000}
            max={999999}
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        ZIP
    </Input>

    <!-- City-->
    <Input
            bind:value={user.user_values.city}
            bind:error={formErrorsValues.city}
            autocomplete="off"
            placeholder="City"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        CITY
    </Input>

    <!-- Country-->
    <Input
            bind:value={user.user_values.country}
            bind:error={formErrorsValues.country}
            autocomplete="off"
            placeholder="Country"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        COUNTRY
    </Input>

    <!-- Phone-->
    <Input
            bind:value={user.user_values.phone}
            bind:error={formErrorsValues.phone}
            autocomplete="off"
            placeholder="Phone"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        PHONE
    </Input>

    <!-- Birthdate-->
    <Input
            type="date"
            bind:value={user.user_values.birthdate}
            bind:error={formErrorsValues.birthdate}
            autocomplete="off"
            placeholder="Birthdate"
            on:keypress={handleKeyPress}
            on:input={validateForm}
    >
        BIRTHDATE
    </Input>

    <!-- Roles-->
    <div class="unit" style:margin-top="-3px">
        <div class="label">
            ROLES
        </div>
        <ItemTiles
                options={allRoles}
                bind:items={user.roles}
                searchThreshold={4}
        />
    </div>

    <!-- Groups-->
    <div class="unit">
        <div class="label">
            GROUPS
        </div>
        <ItemTiles
                options={allGroups}
                bind:items={user.groups}
                searchThreshold={4}
        />
    </div>

    <!-- Limit Lifetime -->
    <div class="unit" style:margin-top="12px">
        <div class="label font-label">
            LIMIT LIFETIME
        </div>
        <div class="value">
            <Switch bind:selected={limitLifetime}/>
        </div>
    </div>
    {#if limitLifetime}
        <Input
                type="datetime-local"
                step="60"
                width="16rem"
                bind:value={userExpires}
                on:input={validateForm}
                min={formatDateToDateInput(new Date())}
                max="2099-01-01T00:00"
        >
            USER EXPIRES
        </Input>
    {/if}

    <!-- Last Login-->
    <div class="unit" style:margin-top="12px">
        <div class="label font-label">
            LAST LOGIN
        </div>
        <div class="value">
            {#if user.last_login}
                {formatDateFromTs(user.last_login)}
            {:else}
                Never
            {/if}
        </div>
    </div>

    <!-- Password Expires-->
    <div class="unit">
        <div class="label font-label">
            PASSWORD EXPIRES
        </div>
        <div class="value">
            {#if user.password_expires}
                {formatDateFromTs(user.password_expires)}
            {:else}
                Never
            {/if}
        </div>
    </div>

    <!-- MFA active -->
    <div class="unit">
        <div class="label font-label">
            MFA ACTIVE
        </div>
        <div class="value">
            <CheckIcon check={!!user.webauthn_user_id}/>
        </div>
    </div>

    <!-- Save Button-->
    <div class="unit">
        <Button on:click={onSubmit} level={1} width="4rem">SAVE</Button>

        {#if success}
            <div class="success">
                Success
            </div>
        {/if}

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        padding: 0 10px 10px 10px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .success {
        margin: 0 5px;
    }

    .row {
        display: flex;
        justify-content: flex-start;
        align-items: center;
    }

    .label {
        margin: 5px 5px 0 5px;
        font-size: .9rem;
    }

    .language {
        display: flex;
        gap: .33rem;
    }

    .success {
        color: var(--col-ok);
    }

    .unit {
        margin: 7px 0;
    }

    .value {
        margin-left: 5px;
        display: flex;
    }
</style>
