<script>
    import PasswordPolicy from "../passwordReset/PasswordPolicy.svelte";
    import * as yup from "yup";
    import {onMount} from "svelte";
    import {getPasswordPolicy} from "../../utils/dataFetching.js";
    import {extractFormErrors, generatePassword} from "../../utils/helpers.js";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import Button from "$lib/Button.svelte";

    export let t;
    export let formValues = {};
    export let btnWidth = "4rem";
    export let inputWidth;

    let accepted = false;
    let err = '';
    let policy;
    let formErrors = {};
    let showCopy;

    const schema = yup.object().shape({
        current: yup.string().required(t.passwordCurrReq),
        new: yup.string()
            .required(t.passwordNewReq),
        verify: yup.string()
            .required(t.passwordNoMatch),
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
            err = t.passwordPolicyFollow;
            return false;
        }

        if (formValues.new !== formValues.verify) {
            err = t.passwordNoMatch;
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
        <PasswordPolicy bind:t bind:accepted bind:policy bind:password={formValues.new}/>

        <PasswordInput
                bind:value={formValues.current}
                bind:error={formErrors.current}
                autocomplete="current-password"
                placeholder={t.passwordCurr}
                on:input={isValid}
                width={inputWidth}
        >
            {t.passwordCurr.toUpperCase()}
        </PasswordInput>
        <PasswordInput
                bind:value={formValues.new}
                bind:error={formErrors.new}
                autocomplete="new-password"
                placeholder={t.passwordNew}
                on:input={isValid}
                bind:showCopy
                width={inputWidth}
        >
            {t.passwordNew.toUpperCase()}
        </PasswordInput>
        <PasswordInput
                bind:value={formValues.verify}
                bind:error={formErrors.verify}
                autocomplete="new-password"
                placeholder={t.passwordConfirm}
                on:input={isValid}
                bind:showCopy
                width={inputWidth}
        >
            {t.passwordConfirm.toUpperCase()}
        </PasswordInput>

        <Button on:click={generate} level={3} width={btnWidth}>
            {t.generateRandom.toUpperCase()}
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
