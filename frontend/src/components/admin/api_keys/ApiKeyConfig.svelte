<script>
    import {run} from 'svelte/legacy';

    import * as yup from "yup";
    import {extractFormErrors, formatUtcTsFromDateInput} from "../../../utils/helpers";
    import Button from "$lib/Button.svelte";
    import {REGEX_API_KEY} from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {slide} from "svelte/transition";
    import Input from "$lib/inputs/Input.svelte";
    import Switch from "$lib/Switch.svelte";
    import ApiKeyAccessMatrix from "./ApiKeyAccessMatrix.svelte";
    import {putApiKey} from "../../../utils/dataFetchingAdmin.js";

    const minDate = new Date().toISOString().split('.')[0];

    let {apiKey = $bindable({}), onSave} = $props();

    let err = $state('');
    let success = $state(false);
    let timer = $state();
    let doesExpire = $state(!!apiKey.expires);
    let accessMatrix = $state();
    // IMPORTANT: do NOT give a default here -> will be initialized inside ApiKeyAccessMatrix!
    let finalizeMatrix = $state();

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

    let formErrors = {};
    let formValues = $state({
        exp: '',
    })

    run(() => {
        if (doesExpire) {
            formValues.exp = new Date().toISOString().split('.')[0];
        }
    });

    async function onSubmit() {
        err = '';

        let data = {
            name: apiKey.name,
            access: finalizeMatrix(),
        };
        if (data.access.length === 0) {
            err = 'Grant the API Key at least one permission';
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

        let res = await putApiKey(data);
        if (res.ok) {
            onSave();
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }
</script>

<div class="container">
    <div class="unit">
        <div class="label">
            Name
        </div>
        <div class="value">
            {apiKey.name}
        </div>
    </div>

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

    <ApiKeyAccessMatrix {apiKey} bind:accessMatrix bind:finalize={finalizeMatrix}/>

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

    .switch {
        display: flex;
        align-items: center;
        gap: .5rem;
        margin: .5rem;
    }

    .unit {
        margin: 7px 0;
    }

    .value {
        margin-left: 5px;
        display: flex;
    }
</style>
