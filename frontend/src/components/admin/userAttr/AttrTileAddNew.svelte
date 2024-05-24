<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_ATTR_DESC, REGEX_ATTR_KEY} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {postAttr} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    export let idx = -1;
    export let onSave;

    let err = '';
    let isLoading = false;
    let success = false;
    let expandContainer;

    let formValues = {
        name: '',
        desc: '',
    };
    let formErrors = {};

    const schema = yup.object().shape({
        name: yup.string().trim().required('Name is required').matches(REGEX_ATTR_KEY, 'Invalid characters: [a-z0-9-_/]{2,32}'),
        desc: yup.string().trim().matches(REGEX_ATTR_DESC, 'Invalid characters: [a-zA-Z0-9\\-_/\\s]{0,128}'),
    });

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            return;
        }
        err = '';
        isLoading = true;

        let res = await postAttr(formValues);
        if (res.ok) {
            expandContainer = false;
            let formValues = {
                name: '',
                desc: '',
            };
            onSave();
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

<ExpandContainer bind:idx bind:show={expandContainer}>
    <div class="header font-label" slot="header">
        ADD NEW USER ATTRIBUTE
    </div>

    <div class="container" slot="body">
        <div class="desc">
            You can add a new custom user attribute.<br>
            These attributes can be set for every user and mapped to an existing scope.<br>
            They are simple Key / JsonValue pairs.
        </div>

        <!-- Name-->
        <Input
                bind:value={formValues.name}
                bind:error={formErrors.name}
                autocomplete="off"
                placeholder="Name"
                on:input={validateForm}
        >
            NAME
        </Input>
        <Input
                bind:value={formValues.desc}
                bind:error={formErrors.desc}
                autocomplete="off"
                placeholder="Description"
                on:input={validateForm}
        >
            DESCRIPTION
        </Input>

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

    .desc {
        margin: 0 5px 15px 5px;
    }

    .header {
        display: flex;
        font-size: .9rem;
        margin-left: 10px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .success {
        color: var(--col-ok);
    }
</style>
