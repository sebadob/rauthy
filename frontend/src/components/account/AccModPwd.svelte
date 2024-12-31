<script>
    import PasswordPolicy from "../passwordReset/PasswordPolicy.svelte";
    import * as yup from "yup";
    import {onMount} from "svelte";
    import {getPasswordPolicy} from "../../utils/dataFetching.js";
    import {extractFormErrors, generatePassword} from "../../utils/helpers.js";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import Button from "$lib/Button.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} t
     * @property {any} [formValues]
     * @property {string} [btnWidth]
     * @property {boolean} [hideCurrentPassword]
     * @property {any} inputWidth
     * @property {() => Promise<boolean>} isValid
     */

    /** @type {Props} */
    let {
        t,
        formValues = $bindable({}),
        btnWidth = "4rem",
        hideCurrentPassword = false,
        inputWidth,
        isValid = $bindable(),
    } = $props();

    isValid = isPwdValid;

    let accepted = $state(false);
    let err = $state('');
    let policy = $state();
    let formErrors = $state({});
    let showCopy = $derived(formValues.new?.length > 6 && formValues.new === formValues.verify);

    const schema = yup.object().shape({
        current: yup.string().required(t.passwordCurrReq),
        new: yup.string()
            .required(t.passwordNewReq),
        verify: yup.string()
            .required(t.passwordNoMatch),
    });
    const schemaWithoutCurrent = yup.object().shape({
        new: yup.string()
            .required(t.passwordNewReq),
        verify: yup.string()
            .required(t.passwordNoMatch),
    });


    onMount(async () => {
        let res = await getPasswordPolicy();
        let body = await res.json();
        if (!res.ok) {
            err = body.message;
        } else {
            policy = body;
        }
    });

    export async function isPwdValid() {
        err = '';

        if (hideCurrentPassword) {
            try {
                await schemaWithoutCurrent.validate(formValues, {abortEarly: false});
                formErrors = {};
            } catch (err) {
                formErrors = extractFormErrors(err);
                return false;
            }
        } else {
            try {
                await schema.validate(formValues, {abortEarly: false});
                formErrors = {};
            } catch (err) {
                formErrors = extractFormErrors(err);
                return false;
            }
        }

        if (!accepted) {
            err = t.passwordPolicyFollow;
            return false;
        }

        if (formValues.new.length > 256) {
            err = 'max 256';
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
        <PasswordPolicy {t} bind:accepted {policy} password={formValues.new}/>

        {#if !hideCurrentPassword}
            <PasswordInput
                    bind:value={formValues.current}
                    error={formErrors.current}
                    autocomplete="current-password"
                    placeholder={t.passwordCurr}
                    on:input={isPwdValid}
                    width={inputWidth}
            >
                {t.passwordCurr.toUpperCase()}
            </PasswordInput>
        {/if}
        <PasswordInput
                bind:value={formValues.new}
                error={formErrors.new}
                autocomplete="new-password"
                placeholder={t.passwordNew}
                on:input={isPwdValid}
                {showCopy}
                width={inputWidth}
        >
            {t.passwordNew.toUpperCase()}
        </PasswordInput>
        <PasswordInput
                bind:value={formValues.verify}
                error={formErrors.verify}
                autocomplete="new-password"
                placeholder={t.passwordConfirm}
                on:input={isPwdValid}
                {showCopy}
                width={inputWidth}
        >
            {t.passwordConfirm.toUpperCase()}
        </PasswordInput>

        <Button on:click={generate} level={2} width={btnWidth}>
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
