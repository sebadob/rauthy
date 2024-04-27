<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import Switch from "$lib/Switch.svelte";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import Button from "$lib/Button.svelte";
    import {
        FLOWS,
        PKCE_CHALLENGES,
        REGEX_CLIENT_NAME,
        REGEX_CONTACT,
        REGEX_URI,
        TOKEN_ALGS
    } from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {globalScopesNames} from "../../../stores/admin.js";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import SwitchList from "$lib/SwitchList.svelte";
    import {putClient} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";
    import ExpandableInput from "$lib/expandableInputs/ExpandableInputs.svelte";
    import {slide} from "svelte/transition";

    export let client = {};
    export let onSave;

    const urlInputWidth = '350px';

    let isLoading = false;
    let err = '';
    let success = false;
    let timer;

    let clientFlows = FLOWS.map(f => {
        if (f.label === 'device_code') {
            f.value = client.flows_enabled?.includes('urn:ietf:params:oauth:grant-type:device_code');
        } else {
            f.value = client.flows_enabled?.includes(f.label);
        }
        return f;
    });

    let allScopes;
    globalScopesNames.subscribe(scps => {
        allScopes = scps;
    })

    let pkceChallenges = PKCE_CHALLENGES.map(c => {
        c.value = client.challenges?.includes(c.label);
        return c;
    });

    let validateContacts;
    let validateAllowedOrigins;
    let validateRedirectUris;
    let validatePostLogoutUris;

    // This hook is needed to not show `undefined` in inputs after some
    // values have been emptied manually
    $: if (client.id) {
        checkUndefinedValues();
    }

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
        name: yup.string().trim().matches(REGEX_CLIENT_NAME, "Can only contain characters, numbers and '-'"),
        client_uri: yup.string().trim().nullable().matches(REGEX_URI, "Invalid URI"),
        access_token_lifetime: yup.number().required('Token Lifetime is required').min(10, 'Cannot be lower than 10').max(86400, 'Cannot be higher than 86400'),
    });

    function checkUndefinedValues() {
        if (client.redirect_uris[0] === '') {
            client.redirect_uris = [];
        }
        if (!client.post_logout_redirect_uris || client.post_logout_redirect_uris[0] === '') {
            client.post_logout_redirect_uris = [];
        }
        if (!client.name) {
            client.name = '';
        }
        if (!client.contacts || client.contacts[0] === '') {
            client.contacts = [];
        }
        if (!client.client_uri) {
            client.client_uri = null;
        }
    }

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        const valid = await validateForm();
        if (!validateAllowedOrigins()) {
            err = 'Invalid Allowed Origin';
            return;
        }
        if (!validateRedirectUris()) {
            err = 'Invalid Redirect URIs';
            return;
        }
        if (!validatePostLogoutUris()) {
            err = 'Invalid Post Logout URIs';
            return;
        }
        if (!validateContacts()) {
            err = 'Invalid Contacts';
            return;
        }
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        client.access_token_lifetime = Number.parseInt(client.access_token_lifetime);
        client.flows_enabled = clientFlows.filter(f => f.value).map(f => {
            if (f.label === 'device_code') {
                // We will not show the full flow name in the UI for nicer UX,
                // but the backend validation is strict.
                return 'urn:ietf:params:oauth:grant-type:device_code';
            } else {
                return f.label;
            }
        });

        if (client.flows_enabled.includes('authorization_code') && client.redirect_uris.length === 0) {
            err = "With 'authorization_code' flow enabled, you need to specify at least one redirect URI";
            return;
        }

        if (client.flows_enabled.includes('client_credentials') && !client.confidential) {
            err = "'client_credentials' flow needs a 'confidential' client";
            return;
        }

        if (client.flows_enabled.length === 0) {
            err = 'At least one flow must be enabled';
            return;
        }

        let data = client;
        client.challenges = pkceChallenges.filter(c => c.value).map(c => c.label);
        if (client.challenges.length === 0) {
            data.challenges = null;
        }

        if (!data.name) {
            data.name = undefined;
        }
        if (data.allowed_origins.length > 0 && !data.allowed_origins[0]) {
            data.allowed_origins = [];
        }

        let res = await putClient(data);
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
            await schema.validate(client, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }
</script>

<div class="container">
    <!-- Client ID -->
    <div class="unit">
        <div class="label font-label">
            CLIENT ID
        </div>
        <div class="value font-mono">
            {client.id}
        </div>
    </div>

    <!-- Client Name -->
    <div class="desc">
        <p>
            The Name can be changed without any impact on the configuration.<br>
            It will just show up on the Login / Logout screens.
        </p>
    </div>
    <Input
            bind:value={client.name}
            bind:error={formErrors.name}
            autocomplete="off"
            placeholder="Client Name"
            on:keypress={handleKeyPress}
            on:input={validateForm}
            width={urlInputWidth}
    >
        CLIENT NAME
    </Input>

    <div class="desc">
        <p>
            Information about this client's URI and some contacts.
            Client URI and Contacts might be shown to users on the login page.
        </p>
    </div>
    <!-- Client URI -->
    <Input
            bind:value={client.client_uri}
            bind:error={formErrors.client_uri}
            autocomplete="off"
            placeholder="Client URI"
            on:keypress={handleKeyPress}
            on:input={validateForm}
            width={urlInputWidth}
    >
        CLIENT URI
    </Input>

    <!-- Contacts -->
    <ExpandableInput
            style="width: {urlInputWidth}"
            validation={{
            required: false,
            regex: REGEX_CONTACT,
            errMsg: "Only URL safe values: a-zA-Z0-9,.:/_-&?=~#!$'()*+%",
        }}
            bind:values={client.contacts}
            bind:validate={validateContacts}
            autocomplete="off"
            placeholder="Contact"
            optional
    >
        CONTACT
    </ExpandableInput>

    <div class="desc">
        <p>
            Client configuration
        </p>
    </div>
    <div class="row" style:margin-top="-5px">
        <!-- Enabled -->
        <div class="unit" style:width="138px">
            <div class="label font-label">
                ENABLED
            </div>
            <div class="value">
                <Switch bind:selected={client.enabled}/>
            </div>
        </div>

        <!-- Confidential -->
        <div class="unit" style:width="110px">
            <div class="label font-label">
                CONFIDENTIAL
            </div>
            <div class="value confidential">
                <Switch bind:selected={client.confidential}/>
            </div>
        </div>
    </div>

    <!-- Flows Enabled -->
    <div class="unit">
        <div class="label font-label">
            FLOWS ENABLED
        </div>
        <div class="flows">
            <SwitchList bind:options={clientFlows}/>
        </div>
    </div>

    <!-- Force MFA -->
    <div class="unit">
        <div class="label font-label">
            FORCE MFA
        </div>
        <div class="value">
            <Switch bind:selected={client.force_mfa}/>
        </div>
    </div>
    {#if client.id === 'rauthy'}
        <div class="desc" style:margin="-10px 0 -15px 5px">
            <p>
                <span style:color="var(--col-err)"><b>CAUTION:</b></span>
                The <code>FORCE MFA</code> option for <code>rauthy</code> itself is managed statically via the
                <code>ADMIN_FORCE_MFA</code> config variable and will be overridden with each restart.
            </p>
        </div>
    {:else if client.force_mfa}
        <div transition:slide class="desc" style:margin="-10px 0 -15px 5px">
            <p>
                <span style:color="var(--col-err)"><b>CAUTION:</b></span>
                If you <code>FORCE MFA</code> for this client here, this will only apply to the
                <code>authorization_code</code> flow! If you use other flows,
                MFA cannot be forced for them!
            </p>
        </div>
    {/if}

    <!-- Scopes Description -->
    <div class="separator"></div>
    <div class="desc">
        <p>
            Allowed Scopes will be applied, if the client asks for them during the login.<br>
            Default Scopes will always be applied.
        </p>
    </div>

    <!-- Allowed Scopes -->
    <div class="unit" style:margin-top="-10px">
        <div class="label">
            ALLOWED SCOPES
        </div>
        <ItemTiles
                options={allScopes}
                bind:items={client.scopes}
                searchThreshold={4}
        />
    </div>

    <!-- Default Scopes -->
    <div class="unit" style:margin-top="-3px">
        <div class="label">
            DEFAULT SCOPES
        </div>
        <ItemTiles
                options={allScopes}
                bind:items={client.default_scopes}
                searchThreshold={4}
        />
    </div>

    <!-- URLs Description -->
    <div class="separator">
    </div>
    <div class="desc">
        <p>
            The allowed origins and the redirect URIs may contain a <code>*</code> wildcard only at the end.
        </p>
    </div>

    <!-- Allowed Origins -->
    <ExpandableInput
            style="width: {urlInputWidth}"
            validation={{
          required: true,
          regex: REGEX_URI,
          errMsg: "Only URL safe values: a-zA-Z0-9,.:/_-&?=~#!$'()*+%",
        }}
            bind:values={client.allowed_origins}
            bind:validate={validateAllowedOrigins}
            autocomplete="off"
            placeholder="Allowed Origin"
            optional
    >
        ALLOWED ORIGIN
    </ExpandableInput>

    <!-- Redirect URIs -->
    <ExpandableInput
            style="width: {urlInputWidth}"
            validation={{
          required: true,
          regex: REGEX_URI,
          errMsg: "Only URL safe values: a-zA-Z0-9,.:/_-&?=~#!$'()*+%",
        }}
            bind:values={client.redirect_uris}
            bind:validate={validateRedirectUris}
            autocomplete="off"
            placeholder="Redirect URI"
            optional
    >
        REDIRECT URI
    </ExpandableInput>

    <!-- Post Logout Redirect URIs -->
    <ExpandableInput
            style="width: {urlInputWidth}"
            validation={{
          required: true,
          regex: REGEX_URI,
          errMsg: "Only URL safe values: a-zA-Z0-9,.:/_-&?=~#!$'()*+%",
        }}
            bind:values={client.post_logout_redirect_uris}
            bind:validate={validatePostLogoutUris}
            autocomplete="off"
            placeholder="Post Logout Redirect URI"
            optional
    >
        POST LOGOUT REDIRECT URI
    </ExpandableInput>

    <!-- Tokens Description -->
    <div class="separator"></div>
    <div class="desc">
        <p>
            The Token Lifetime applies to the Access and ID tokens and is specified in seconds.
        </p>
        <p>
            If your client does support EdDSA / ed25519 token algorithms, you should always use it for better security
            and
            smaller tokens.<br>
            The RSA algorithms does exist for compatibility reasons only.
        </p>
        <p>
            The algorithm for the optional refresh token cannot be changed, since this should only be used by
            <i>rauthy</i> anyway.
        </p>
    </div>

    <!-- Token Lifetime -->
    <Input
            type="number"
            min={60}
            max={86400}
            bind:value={client.access_token_lifetime}
            bind:error={formErrors.access_token_lifetime}
            autocomplete="off"
            placeholder="Client Name"
            on:input={validateForm}
            width={urlInputWidth}
    >
        ACCESS TOKEN LIFETIME
    </Input>

    <div class="row">
        <!-- Access Token Alg -->
        <div class="unit" style:width="220px">
            <div class="label font-label">
                ACCESS ALGORITHM
            </div>
            <div class="value">
                <OptionSelect bind:value={client.access_token_alg} options={TOKEN_ALGS}/>
            </div>
        </div>

        <!-- ID Token Alg -->
        <div class="unit">
            <div class="label font-label" style:text-align="right">
                ID ALGORITHM
            </div>
            <div class="value">
                <OptionSelect bind:value={client.id_token_alg} options={TOKEN_ALGS}/>
            </div>
        </div>
    </div>

    <!-- Refresh Tokens -->
    <div class="unit">
        <div class="label font-label">
            REFRESH TOKENS
        </div>
        <div class="value">
            <Switch bind:selected={client.refresh_token}/>
        </div>
    </div>

    <!-- PKCE Description -->
    <div class="separator">
    </div>
    <div class="desc">
        <p>
            If your application does support it, you should always use S256 PKCE challenges.<br>
            If you login from a single page application directly without any backend and therefore have a
            non-confidential
            client, you must(!) use at least the plain PKCE challenge, to have a secure login flow.
        </p>
        <p>
            If any of these is set, <i>rauthy</i> will enforce the usage and deny any login, which does not provide a
            valid challenge.
        </p>
    </div>

    <!-- PKCE Challenges -->
    <div class="unit">
        <div class="label font-label">
            PKCE CHALLENGES
        </div>
        <div class="challenges">
            <SwitchList bind:options={pkceChallenges}/>
        </div>
    </div>


    <!-- Warning message for 'rauthy' -->
    <div class="desc err">
        {#if client.id === 'rauthy'}
            <div class="separator"></div>
            <b>CAUTION:</b>
            <p>
                <code>rauthy</code> is the default client which is needed for logging into this Admin UI.<br>
                Be VERY careful when you change values here, since you could end up locking yourself out of the UI.
            </p>
        {/if}
    </div>

    <!-- Save Button-->
    <Button on:click={onSubmit} level={1} width="4rem">SAVE</Button>

    {#if success}
        <div class="success">
            Success
        </div>
    {/if}

    {#if err}
        <div class="mainErr err">
            {err}
        </div>
    {/if}
</div>

<style>
    code {
        font-size: .95rem;
    }

    .challenges {
        width: 140px;
    }

    .confidential {
        justify-content: flex-end;
        margin-right: 10px;
    }

    .container {
        padding: 0 15px 10px 15px;
    }

    .desc {
        margin: 10px 5px;
    }

    .err {
        color: var(--col-err);
    }

    .mainErr, .success {
        display: flex;
        align-items: center;
        margin: 0 10px;
    }

    .flows {
        width: 250px;
    }

    .label {
        margin-top: 5px;
        font-size: .9rem;
    }

    .separator {
        height: 10px;
        width: 30px;
    }

    .row {
        display: flex;
        justify-content: flex-start;
        align-items: center;
    }

    .success {
        color: var(--col-ok);
    }

    .unit {
        margin: 7px 5px;
    }

    .value {
        display: flex;
        align-items: center;
    }
</style>
