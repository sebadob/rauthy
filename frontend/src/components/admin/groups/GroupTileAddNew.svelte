<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postGroup} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    export let idx = -1;
    export let onSave;
    let expandContainer;

    let group = {group: ''};

    let err = '';
    let isLoading = false;
    let success = false;
    let timer;
    let formErrors = {};

    const schema = yup.object().shape({
        group: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/:*', length: 2-64"),
    });

    $: if (success) {
        timer = setTimeout(() => {
            onSave();
            success = false;
            group = {group: ''};
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
    <div class="header font-label" slot="header">
        ADD NEW GROUP
    </div>

    <div class="container" slot="body">
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
