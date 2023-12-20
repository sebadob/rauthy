<script>
    import * as yup from "yup";
    import {
        extractFormErrors,
        formatDateFromTs,
        formatDateToDateInput,
        formatUtcTsFromDateInput
    } from "../../../utils/helpers.js";
    import Switch from "$lib/Switch.svelte";
    import {globalGroupsNames, globalRolesNames} from "../../../stores/admin.js";
    import Button from "$lib/Button.svelte";
    import {LANGUAGES, REGEX_NAME} from "../../../utils/constants.js";
    import {putUser} from "../../../utils/dataFetchingAdmin.js";
    import {onMount} from "svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";

    export let user = {};
    export let onSave;

    let isLoading = false;
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

    onMount(() => {
        return () => clearTimeout(timer);
    });

    let formErrors = {};
    const schema = yup.object().shape({
        email: yup.string().required('E-Mail is required').email("Bad E-Mail format"),
        given_name: yup.string().trim().required('Given Name is required').matches(REGEX_NAME, 'Invalid characters'),
        family_name: yup.string().trim().required('Family Name is required').matches(REGEX_NAME, 'Invalid characters'),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            return;
        }
        err = '';
        isLoading = true;

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
        };

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
        isLoading = false;
    }

    async function validateForm() {
        try {
            await schema.validate(user, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
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
