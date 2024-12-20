<script>
    import { run } from 'svelte/legacy';

    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postGroup} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    let { idx = $bindable(-1), onSave } = $props();
    let expandContainer = $state();

    let group = $state({group: ''});

    let err = $state('');
    let isLoading = false;
    let success = $state(false);
    let timer = $state();
    let formErrors = $state({});

    const schema = yup.object().shape({
        group: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/:*', length: 2-64"),
    });

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                onSave();
                success = false;
                group = {group: ''};
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
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        group.group = group.group.trim();
        let res = await postGroup(group);
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
            await schema.validate(group, {abortEarly: false});
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
            ADD NEW GROUP
        </div>
    {/snippet}

    {#snippet body()}
        <div class="container" >
            <Input
                    bind:value={group.group}
                    bind:error={formErrors.group}
                    autocomplete="off"
                    placeholder="Group Name"
                    on:input={validateForm}
            >
                GROUP NAME
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
