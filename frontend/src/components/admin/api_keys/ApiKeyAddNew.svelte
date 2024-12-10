<script>
    import { run } from 'svelte/legacy';

    import Button from "$lib/Button.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import * as yup from "yup";
    import {REGEX_API_KEY} from "../../../utils/constants.js";
    import {postApiKey} from "../../../utils/dataFetchingAdmin.js";
    import Switch from "$lib/Switch.svelte";
    import {slide} from "svelte/transition";
    import {extractFormErrors, formatUtcTsFromDateInput} from "../../../utils/helpers.js";
    import ApiKeyAccessMatrix from "./ApiKeyAccessMatrix.svelte";

    const minDate = new Date().toISOString().split('.')[0];

    let { apiKeys = [], onSave = () => {} } = $props();

    let err = $state('');
    let doesExpire = $state(false);
    // IMPORTANT: do NOT give a default here -> will be initialized inside ApiKeyAccessMatrix!
    let finalizeMatrix = $state();

    let accessMatrix = $state();
    let formValues = $state({
        name: '',
        exp: '',
    })
    let formErrors = $state({});
    const schema = yup.object().shape({
        name: yup.string()
            .required('Name is required')
            .min(2)
            .max(24)
            .matches(REGEX_API_KEY, 'Format: [a-zA-Z0-9_/-]{2,24}'),
    });

    run(() => {
        if (doesExpire) {
            formValues.exp = new Date().toISOString().split('.')[0];
        }
    });

    async function onSubmit() {
        err = '';

        const isValid = await validateForm();
        if (!isValid) {
            return;
        }

        let data = {
            name: formValues.name,
            access: finalizeMatrix(),
        };
        if (data.access.length === 0) {
            err = 'Grant the new API Key at least one permission';
            return
        }

        if (doesExpire) {
            let exp = formatUtcTsFromDateInput(formValues.exp);
            if (!exp) {
                err = 'Invalid Date Input: User Expires';
                return;
            }
            data.exp = exp;
        }

        let res = await postApiKey(data);
        if (res.ok) {
            onSave();
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

    async function validateForm() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }

        for (let key of apiKeys) {
            if (key.name === formValues.name) {
                formErrors.name = 'Name already exists';
                return false;
            }
        }

        return true;
    }

</script>

<div class="container">
    <Input
            width="18rem"
            bind:value={formValues.name}
            bind:error={formErrors.name}
            autocomplete="off"
            placeholder="Name"
    >
        NAME
    </Input>

    <div class="switch">
        Key Expires:
        <Switch bind:selected={doesExpire}/>
    </div>

    {#if doesExpire}
        <div transition:slide>
            <Input
                    type="datetime-local"
                    step="60"
                    width="18rem"
                    bind:value={formValues.exp}
                    min={minDate}
                    max="2099-01-01T00:00"
            >
                EXPIRES
            </Input>
        </div>
    {/if}

    <ApiKeyAccessMatrix bind:accessMatrix bind:finalize={finalizeMatrix} />

    <div class="saveBtn">
        <Button on:click={onSubmit} level={1}>SAVE</Button>
    </div>

    <div class="err">
        {err}
    </div>
</div>

<style>
    .container {
        margin-bottom: 1rem;
    }

    .err {
        color: var(--col-err);
    }

    .saveBtn {
        margin-top: .25rem;
    }

    .switch {
        display: flex;
        align-items: center;
        gap: .5rem;
        margin: .5rem;
    }
</style>
