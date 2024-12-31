<script>
    import { run } from 'svelte/legacy';

    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postRole} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    let { idx = $bindable(-1), onSave } = $props();
    let expandContainer = $state();

    let role = $state({role: ''});

    let err = $state('');
    let isLoading = false;
    let success = $state(false);
    let timer = $state();
    let formErrors = $state({});

    const schema = yup.object().shape({
        role: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/:*', length: 2-64"),
    });

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                success = false;
                role = {role: ''};
                expandContainer = false;
                onSave();
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
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        role.role = role.role.trim();
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
    {#snippet header()}
        <div class="header font-label" >
            ADD NEW ROLE
        </div>
    {/snippet}

    {#snippet body()}
        <div class="container" >
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

    .success {
        color: var(--col-ok);
    }
</style>
