<script>
    import PasswordPolicy from "../passwordReset/PasswordPolicy.svelte";
    import * as yup from "yup";
    import {onMount} from "svelte";
    import {getPasswordPolicy} from "../../utils/dataFetching.js";
    import {extractFormErrors, generatePassword} from "../../utils/helpers.js";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import Button from "$lib/Button.svelte";

    export let formValues = {};
    export let btnWidth = 170;
    export let inputWidth;

    let accepted = false;
    let err = '';
    let policy;
    let formErrors = {};
    let showCopy;

    const schema = yup.object().shape({
        current: yup.string().required('Current password is required'),
        new: yup.string()
            .required('New password is required'),
        verify: yup.string()
            .required('Password verification is required'),
    });

    $: showCopy = formValues.new?.length > 6 && formValues.new === formValues.verify;

    onMount(async () => {
        let res = await getPasswordPolicy();
        let body = await res.json();
        if (!res.ok) {
            err = body.message;
        } else {
            policy = body;
        }
    });

    export async function isValid() {
        err = '';

        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }

        if (!accepted) {
            err = 'You must follow the password policy';
            return false;
        }

        if (formValues.new !== formValues.verify) {
            err = 'New passwords do not match';
            return false;
        }

        return true;
    }

    function generate() {
        const len = policy.length_min > 20 ? policy.length_min : 20;
        let pwd = generatePassword(
            len, policy.include_lower_case, policy.include_upper_case, policy.include_digits, policy.include_special
        );
        formValues.new = pwd;
        formValues.verify = pwd;
    }

</script>

{#if policy}
    <div class="container">
        <PasswordPolicy bind:accepted bind:policy bind:password={formValues.new}/>

        <PasswordInput
                bind:value={formValues.current}
                bind:error={formErrors.current}
                autocomplete="current-password"
                placeholder="Current Password"
                on:input={isValid}
                width={inputWidth}
        >
            CURRENT PASSWORD
        </PasswordInput>
        <PasswordInput
                bind:value={formValues.new}
                bind:error={formErrors.new}
                autocomplete="new-password"
                placeholder="New Password"
                on:input={isValid}
                bind:showCopy
                width={inputWidth}
        >
            NEW PASSWORD
        </PasswordInput>
        <PasswordInput
                bind:value={formValues.verify}
                bind:error={formErrors.verify}
                autocomplete="new-password"
                placeholder="New Password"
                on:input={isValid}
                bind:showCopy
                width={inputWidth}
        >
            CONFIRM PASSWORD
        </PasswordInput>

        <Button on:click={generate} level={3} width={btnWidth}>
            GENERATE RANDOM
        </Button>

        <div class="err">
            {err}
        </div>
    </div>
{/if}

<style>
    .container {
        margin-top: 5px;
        margin-bottom: 10px;
    }

    .err {
        margin: -5px 10px 5px 10px;
        text-align: left;
        color: var(--col-err)
    }
</style>
