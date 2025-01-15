<script>
    import PasswordPolicy from "../passwordReset/PasswordPolicy.svelte";
    import * as yup from "yup";
    import {onMount} from "svelte";
    import {getPasswordPolicy} from "../../utils/dataFetching.js";
    import {extractFormErrors, generatePassword} from "../../utils/helpers";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import Button from "$lib/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [formValues]
     * @property {string} [btnWidth]
     * @property {boolean} [hideCurrentPassword]
     * @property {any} inputWidth
     * @property {() => Promise<boolean>} isValid
     */

    /** @type {Props} */
    let {
        formValues = $bindable({}),
        btnWidth = "4rem",
        hideCurrentPassword = false,
        inputWidth,
        isValid = $bindable(),
    } = $props();

    let t = useI18n();

    isValid = isPwdValid;

    let accepted = $state(false);
    let err = $state('');
    let policy = $state();
    let formErrors = $state({});
    let showCopy = $derived(formValues.new?.length > 6 && formValues.new === formValues.verify);

    const schema = yup.object().shape({
        current: yup.string().required(t.account.passwordCurrReq),
        new: yup.string()
            .required(t.account.passwordNewReq),
        verify: yup.string()
            .required(t.account.passwordNoMatch),
    });
    const schemaWithoutCurrent = yup.object().shape({
        new: yup.string()
            .required(t.account.passwordNewReq),
        verify: yup.string()
            .required(t.account.passwordNoMatch),
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
            err = t.account.passwordPolicyFollow;
            return false;
        }

        if (formValues.new.length > 256) {
            err = 'max 256';
            return false;
        }

        if (formValues.new !== formValues.verify) {
            err = t.account.passwordNoMatch;
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
                    placeholder={t.account.passwordCurr}
                    on:input={isPwdValid}
                    width={inputWidth}
            >
                {t.account.passwordCurr.toUpperCase()}
            </PasswordInput>
        {/if}
        <PasswordInput
                bind:value={formValues.new}
                error={formErrors.new}
                autocomplete="new-password"
                placeholder={t.account.passwordNew}
                on:input={isPwdValid}
                {showCopy}
                width={inputWidth}
        >
            {t.account.passwordNew.toUpperCase()}
        </PasswordInput>
        <PasswordInput
                bind:value={formValues.verify}
                error={formErrors.verify}
                autocomplete="new-password"
                placeholder={t.account.passwordConfirm}
                on:input={isPwdValid}
                {showCopy}
                width={inputWidth}
        >
            {t.account.passwordConfirm.toUpperCase()}
        </PasswordInput>

        <Button on:click={generate} level={2} width={btnWidth}>
            {t.account.generateRandom}
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
