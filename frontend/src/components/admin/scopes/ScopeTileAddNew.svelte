<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postScope} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    export let idx = -1;
    export let onSave;
    let expandContainer;

    let scope = {
        scope: '',
    }

    let err = '';
    let isLoading = false;
    let success = false;
    let timer;
    let formErrors = {};

    const schema = yup.object().shape({
        scope: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/', length: 2-128"),
    });

    $: if (success) {
        timer = setTimeout(() => {
            onSave();
            success = false;
            scope = {};
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
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        let res = await postScope(scope);
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
            await schema.validate(scope, {abortEarly: false});
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
        ADD NEW SCOPE
    </div>

    <div class="container" slot="body">
        <Input
                bind:value={scope.scope}
                bind:error={formErrors.scope}
                autocomplete="off"
                placeholder="Scope Name"
                on:input={validateForm}
        >
            SCOPE NAME
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
