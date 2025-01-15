<script>
    import {run} from 'svelte/legacy';

    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers";
    import Button from "$lib/Button.svelte";
    import {REGEX_ATTR_DESC, REGEX_ATTR_KEY} from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {putAttr} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    let {attr = {}, onSave = $bindable()} = $props();

    let isLoading = false;
    let err = $state('');
    let success = $state(false);
    let timer = $state();

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                success = false;
                onSave();
            }, 2000);
        }
    });

    onMount(() => {
        return () => clearTimeout(timer);
    });

    let formErrors = $state({});
    let formValues = $state({});

    const schema = yup.object().shape({
        name: yup.string().trim().required('Name is required').matches(REGEX_ATTR_KEY, 'Valid characters: [a-z0-9-_/]{2,32}'),
        desc: yup.string().nullable().trim().matches(REGEX_ATTR_DESC, 'Valid characters: [a-zA-Z0-9\\-_/\\s]{0,128}'),
    });

    onMount(() => {
        formValues.name = attr.name;
        formValues.desc = attr.desc;
    })

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

        let data = {
            name: formValues.name,
            desc: formValues.desc,
        };

        let res = await putAttr(attr.name, data);
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
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }
</script>

<div class="container">
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
        color: var(--col-ok);
    }
</style>
