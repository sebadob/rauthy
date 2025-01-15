<script>
    import {run} from 'svelte/legacy';

    import * as yup from "yup";
    import Button from "$lib/Button.svelte";
    import {extractFormErrors} from "../../../../utils/helpers";
    import {postPasswordHashTimes} from "../../../../utils/dataFetchingAdmin.js";
    import Argon2Results from "./Argon2Results.svelte";
    import Input from "$lib/inputs/Input.svelte";

    const inputWidth = '170px';

    let isLoading = $state(false);
    let err = '';
    let success = $state(false);
    let hashTimes = $state();
    let scrollY = $state();

    let formValues = $state({
        targetTime: 1000,
        mCost: 32768,
        pCost: 2,
    });
    let formErrors = $state({});

    const schema = yup.object().shape({
        targetTime: yup.number()
            .required('Target Time is required')
            .min(500, 'Cannot be lower than 500')
            .max(99999, 'Cannot be higher than 99999'),
        mCost: yup.number()
            .min(32768, 'Cannot be lower than 32768')
            .max(1048576, 'Cannot be higher than 1048576'),
        pCost: yup.number()
            .min(2, 'Cannot be lower than 2')
            .max(1024, 'Cannot be higher than 1024'),
    });

    run(() => {
        if (success && hashTimes) {
            scrollY = window.innerHeight;
        }
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        // validate form
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        isLoading = true;
        success = false;

        // build payload
        const data = {
            target_time: Number.parseInt(formValues.targetTime),
            m_cost: Number.parseInt(formValues.mCost),
            p_cost: Number.parseInt(formValues.pCost),
        };

        const res = await postPasswordHashTimes(data);
        const body = await res.json();
        if (res.ok) {
            hashTimes = body.results;
            err = '';
            success = true;
        } else {
            err = body.message;
        }

        isLoading = false;
    }
</script>

<svelte:window bind:scrollY={scrollY}/>

<div class="wrapper">
    <div class="container">
        <Input
                type="number"
                bind:value={formValues.targetTime}
                bind:error={formErrors.targetTime}
                on:keypress={handleKeyPress}
                autocomplete="off"
                placeholder="Target Time (ms)"
                width={inputWidth}
        >
            TARGET TIME (ms)
        </Input>

        <Input
                type="number"
                bind:value={formValues.mCost}
                bind:error={formErrors.mCost}
                on:keypress={handleKeyPress}
                autocomplete="off"
                placeholder="m_cost (kB)"
                width={inputWidth}
        >
            M_COST (kB)
        </Input>

        <Input
                type="number"
                bind:value={formValues.pCost}
                bind:error={formErrors.pCost}
                on:keypress={handleKeyPress}
                autocomplete="off"
                placeholder="p_cost"
                width={inputWidth}
        >
            P_COST
        </Input>

        <Button on:click={onSubmit} isLoading={isLoading} level={1}>CALCULATE</Button>
    </div>

    {#if success}
        <div class="result">
            <Argon2Results res={hashTimes}/>
        </div>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .result {
        margin-top: 5px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .wrapper {
        display: flex;
    }
</style>
