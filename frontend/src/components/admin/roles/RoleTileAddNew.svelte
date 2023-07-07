<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postRole} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    export let idx = -1;
    export let onSave;
    let expandContainer;

    let role = {
        role: '',
    }

    let err = '';
    let isLoading = false;
    let success = false;
    let timer;
    let formErrors = {};

    const schema = yup.object().shape({
        role: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/', length: 2-128"),
    });

    $: if (success) {
        timer = setTimeout(() => {
            success = false;
            role = {};
            expandContainer = false;
            onSave();
        }, 1500);
    }

    onMount(() => {
        return () => clearTimeout(timer);
    });

    async function onSubmit() {
        err = '';
        isLoading = true;

        const valid = await validateForm();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        let res = await postRole(role);
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

<ExpandContainer bind:idx bind:show={expandContainer}>
    <div class="header font-label" slot="header">
        ADD NEW ROLE
    </div>

    <div class="container" slot="body">
        <Input
                bind:value={role.role}
                bind:error={formErrors.role}
                autocomplete="off"
                placeholder="Role Name"
                on:input={validateForm}
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
            <div class="err">
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
        font-size: .9rem;
        margin-left: 10px;
    }

    .success {
        color: var(--col-ok);
    }
</style>
