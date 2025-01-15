<script>
    import {run} from 'svelte/legacy';

    import {onMount} from "svelte";
    import {getPasswordPolicy} from "../../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import {extractFormErrors} from "../../../../utils/helpers";
    import * as yup from "yup";
    import {putPasswordPolicy} from "../../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";

    const inputWidth = '160px';

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let timer = $state();
    let policy = $state();

    let formErrors = $state({});
    const schema = yup.object().shape({
        length_min: yup.number()
            .required('Min Length is required')
            .min(8, 'Cannot be lower than 8')
            .max(128, 'Cannot be higher than 128'),
        length_max: yup.number()
            .required('Max Length is required')
            .min(8, 'Cannot be lower than 8')
            .max(128, 'Cannot be higher than 128'),
        not_recently_used: yup.number().nullable()
            .min(0, 'Cannot be lower than 0')
            .max(10, 'Cannot be higher than 10'),
        valid_days: yup.number().nullable()
            .min(0, 'Cannot be lower than 0')
            .max(3650, 'Cannot be higher than 3650'),
        include_digits: yup.number().nullable()
            .min(0, 'Cannot be lower than 0')
            .max(32, 'Cannot be higher than 32'),
        include_lower_case: yup.number().nullable()
            .min(0, 'Cannot be lower than 0')
            .max(32, 'Cannot be higher than 32'),
        include_upper_case: yup.number().nullable()
            .min(0, 'Cannot be lower than 0')
            .max(32, 'Cannot be higher than 32'),
        include_special: yup.number().nullable()
            .min(0, 'Cannot be lower than 0')
            .max(32, 'Cannot be higher than 32'),
    });

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                success = false;
            }, 3000);
        }
    });

    onMount(async () => {
        if (!policy) {
            let res = await getPasswordPolicy();
            let body = await res.json();
            if (!res.ok) {
                err = body.message;
            } else {
                policy = body;
                policy.length_min = body.length_min || 0;
                policy.length_max = body.length_max || 0;
                policy.include_digits = body.include_digits || 0;
                policy.include_lower_case = body.include_lower_case || 0;
                policy.include_upper_case = body.include_upper_case || 0;
                policy.include_special = body.include_special || 0;
                policy.length_max = body.length_max || 0;
                policy.valid_days = body.valid_days || 0;
                policy.not_recently_used = body.not_recently_used || 0;
            }
        }
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            return;
        }

        err = '';
        isLoading = true;

        let data = policy;

        if (data.not_recently_used === 0) {
            data.not_recently_used = null;
        }
        if (data.valid_days === 0) {
            data.valid_days = null;
        }
        if (data.include_digits === 0) {
            data.include_digits = null;
        }
        if (data.include_lower_case === 0) {
            data.include_lower_case = null;
        }
        if (data.include_upper_case === 0) {
            data.include_upper_case = null;
        }
        if (data.include_special === 0) {
            data.include_special = null;
        }

        let res = await putPasswordPolicy(data);
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
            formErrors = {};
            await schema.validate(policy, {abortEarly: false});

            policy.length_min = Number.parseInt(policy.length_min);
            policy.length_max = Number.parseInt(policy.length_max);
            policy.include_digits = Number.parseInt(policy.include_digits);
            policy.include_lower_case = Number.parseInt(policy.include_lower_case);
            policy.include_upper_case = Number.parseInt(policy.include_upper_case);
            policy.include_special = Number.parseInt(policy.include_special);
            policy.length_max = Number.parseInt(policy.length_max);
            policy.valid_days = Number.parseInt(policy.valid_days);
            policy.not_recently_used = Number.parseInt(policy.not_recently_used);

            if (policy.length_max < policy.length_min) {
                formErrors.length_max = 'Max Length cannot be lower than Min Length';
                return false;
            }

            let sum = policy.include_digits + policy.include_lower_case + policy.include_upper_case
                + policy.include_special;
            if (sum > policy.length_max) {
                formErrors.length_max = 'The sum of all includes does not fit into Max Length';
                return false;
            }

            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

</script>

<div class="wrapper">
    <div class="desc">
        <h3>Password Policy</h3>
        <p>
            Configure the password policy.<br>
            The policy is being applied to all passwords being set from this moment on.
        </p>
    </div>

    {#if policy}
        <div class="row">
            <!-- Min Length -->
            <Input
                    type="number"
                    bind:value={policy.length_min}
                    bind:error={formErrors.length_min}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                MIN LENGTH
            </Input>

            <!-- Max Length -->
            <Input
                    type="number"
                    bind:value={policy.length_max}
                    bind:error={formErrors.length_max}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                MAX LENGTH
            </Input>
        </div>

        <div class="row">
            <!-- Include lowercase -->
            <Input
                    type="number"
                    bind:value={policy.include_lower_case}
                    bind:error={formErrors.include_lower_case}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                INCLUDE LOWERCASE
            </Input>

            <!-- Include Uppercase -->
            <Input
                    type="number"
                    bind:value={policy.include_upper_case}
                    bind:error={formErrors.include_upper_case}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                INCLUDE UPPERCASE
            </Input>
        </div>

        <div class="row">
            <!-- Include digits -->
            <Input
                    type="number"
                    bind:value={policy.include_digits}
                    bind:error={formErrors.include_digits}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                INCLUDE DIGITS
            </Input>

            <!-- Include Special -->
            <Input
                    type="number"
                    bind:value={policy.include_special}
                    bind:error={formErrors.include_special}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                INCLUDE SPECIAL
            </Input>
        </div>

        <div class="desc">
            <p>
                Validity for a new password.<br>
                Not Recently Used denied the last n used passwords.<br>
                Setting no value at all with disable the given option.
            </p>
        </div>

        <div class="row">
            <!-- Not Recently Used -->
            <Input
                    type="number"
                    bind:value={policy.not_recently_used}
                    bind:error={formErrors.not_recently_used}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                NOT RECENTLY USED
            </Input>

            <!-- Valid Days -->
            <Input
                    type="number"
                    bind:value={policy.valid_days}
                    bind:error={formErrors.valid_days}
                    on:keypress={handleKeyPress}
                    on:input={validateForm}
                    autocomplete="off"
                    width={inputWidth}
            >
                VALID FOR DAYS
            </Input>
        </div>

        <!-- Save Button -->
        <Button on:click={onSubmit} bind:isLoading level={1} width="4rem">SAVE</Button>

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
    {/if}
</div>

<style>
    .desc {
        margin: 20px 5px 10px 5px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 5px;
    }

    .row {
        display: flex;
    }

    .success {
        color: var(--col-ok);
    }

    .wrapper {
        margin: 0 5px;
    }
</style>
