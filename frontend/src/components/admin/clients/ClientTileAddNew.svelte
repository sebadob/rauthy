<script>
    import { run } from 'svelte/legacy';

    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_CLIENT_ID, REGEX_CLIENT_NAME, REGEX_URI} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers";
    import ExpandableInput from "$lib/expandableInputs/ExpandableInputs.svelte";
    import Switch from "$lib/Switch.svelte";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postClient} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    let { idx = $bindable(-1), onSave } = $props();
    let expandContainer = $state();

    let client = $state({
        id: '',
        name: '',
        confidential: true,
        redirect_uris: [],
    })

    let err = $state('');
    let isLoading = false;
    let success = $state(false);
    let timer = $state();

    let formErrors = $state({});

    const schema = yup.object().shape({
        id: yup.string().required('Client ID is required').trim().matches(REGEX_CLIENT_ID, "Can only contain characters, numbers and '-'"),
        name: yup.string().trim().nullable().matches(REGEX_CLIENT_NAME, "Can only contain characters, numbers and '-'"),
    });

    // will bind to the validation function inside the ExpandableFormInputs component
    let validateRedirectUris = $state();

    // will bind to the validation function inside the ExpandableFormInputs component
    let validatePostLogoutUris = $state();

    const urlInputWidth = '330px';

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                onSave();
                success = false;
                expandContainer = false;
            }, 1500);
        }
    });

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
    {#snippet header()}
        <div class="header font-label" >
            ADD NEW CLIENT
        </div>
    {/snippet}

    {#snippet body()}
        <div class="container" >
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
    {/snippet}
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
        font-size: .9rem;
        margin-left: 10px;
    }

    .label {
        margin: 5px 5px 0 5px;
        font-size: .9rem;
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
