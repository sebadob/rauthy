<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_CLIENT_ID, REGEX_CLIENT_NAME, REGEX_URI} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import ExpandableInput from "$lib/expandableInputs/ExpandableInputs.svelte";
    import Switch from "$lib/Switch.svelte";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postClient} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    export let idx = -1;
    export let onSave;
    let expandContainer;

    let client = {
        id: '',
        name: '',
        confidential: true,
        redirect_uris: [],
    }

    let err = '';
    let isLoading = false;
    let success = false;
    let timer;

    let formErrors = {};

    const schema = yup.object().shape({
        id: yup.string().required('Client ID is required').trim().matches(REGEX_CLIENT_ID, "Can only contain characters, numbers and '-'"),
        name: yup.string().trim().nullable().matches(REGEX_CLIENT_NAME, "Can only contain characters, numbers and '-'"),
    });

    // will bind to the validation function inside the ExpandableFormInputs component
    let validateRedirectUris;

    // will bind to the validation function inside the ExpandableFormInputs component
    let validatePostLogoutUris;

    const urlInputWidth = '330px';

    $: if (success) {
        timer = setTimeout(() => {
            onSave();
            success = false;
            expandContainer = false;
        }, 1500);
    }

    onMount(() => {
        return () => clearTimeout(timer);
    });

    async function onSubmit() {
        err = '';
        isLoading = true;

        const valid = await validateForm();
        if (!valid || !validateRedirectUris() || !validatePostLogoutUris()) {
            err = 'Invalid input';
            return;
        }

        let data = client;
        if (!data.name) {
            data.name = undefined;
        }

        let res = await postClient(data);
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

<ExpandContainer bind:idx bind:show={expandContainer}>
    <div class="header font-label" slot="header">
        ADD NEW CLIENT
    </div>

    <div class="container" slot="body">
        <Input
                bind:value={client.id}
                bind:error={formErrors.id}
                autocomplete="off"
                placeholder="Client ID"
                on:input={validateForm}
                width={urlInputWidth}
        >
            CLIENT ID
        </Input>
        <Input
                bind:value={client.name}
                bind:error={formErrors.name}
                autocomplete="off"
                placeholder="Client Name"
                on:input={validateForm}
                width={urlInputWidth}
        >
            NAME
        </Input>

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
                optional
                placeholder="Redirect URI"
        >
            REDIRECT URI
        </ExpandableInput>
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
                optional
                placeholder="Post Logout Redirect URI"
        >
            POST LOGOUT REDIRECT URI
        </ExpandableInput>

        <div class="unit">
            <div class="label font-label">
                CONFIDENTIAL
            </div>
            <div class="value">
                <Switch bind:selected={client.confidential}/>
            </div>
        </div>

        <Button on:click={onSubmit} level={1}>SAVE</Button>

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
</ExpandContainer>

<style>
    .container {
        padding: 10px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .header {
        display: flex;
        font-size: .9em;
        margin-left: 10px;
    }

    .label {
        margin: 5px 5px 0 5px;
        font-size: .9em;
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
