<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {putRole} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    export let role = {};
    export let onSave;

    let isLoading = false;
    let err = '';
    let success = false;
    let timer;

    $: if (success) {
        timer = setTimeout(() => {
            success = false;
            onSave();
        }, 2000);
    }

    onMount(() => {
        return () => clearTimeout(timer);
    });

    let formErrors = {};
    const schema = yup.object().shape({
        name: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/', length: 2-128"),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        const valid = await validateForm();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        let req = {
            role: role.name
        }

        let res = await putRole(role.id, req);
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
            await schema.validate(role, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }
</script>

<div class="container">
    <div class="unit">
        <div class="label font-label">
            ID
        </div>
        <div class="value font-mono">
            {role.id}
        </div>
    </div>

    <Input
            bind:value={role.name}
            bind:error={formErrors.name}
            autocomplete="off"
            placeholder="Role Name"
            on:input={validateForm}
            disabled={role.name === 'rauthy_admin'}
    >
        ROLE NAME
    </Input>

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
