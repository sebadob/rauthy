<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../utils/helpers.js";
    import {REGEX_NAME} from "../../utils/constants.js";
    import Button from "$lib/Button.svelte";
    import {blur, fade} from 'svelte/transition';
    import {tweened} from 'svelte/motion';
    import AccModPwd from "./AccModPwd.svelte";
    import {putUserSelf} from "../../utils/dataFetching.js";
    import Input from "$lib/inputs/Input.svelte";

    export let user = {};

    const btnWidth = "10rem";
    const inputWidth = '300px';

    let editPwd = false;
    let pwdFormValues = {};
    let isPwdValid;

    let isLoading = false;
    let err = '';
    let success = false;

    let pwdContainerHeight = tweened(0, {
        duration: 200,
        delay: 200,
    });
    $: pwdContainerHeight.set(editPwd ? 360 : 0);

    let formValues = {
        email: user.email,
        givenName: user.given_name,
        familyName: user.family_name,
    };
    let formErrors = {};

    const schema = yup.object().shape({
        email: yup.string().required('E-Mail is required').email("Bad E-Mail format"),
        givenName: yup.string()
            .required('Given Name is required')
            .matches(REGEX_NAME, "Your given name with 2 - 32  non-special characters"),
        familyName: yup.string()
            .required('Family Name is required')
            .matches(REGEX_NAME, "Your family name with 2 - 32  non-special characters"),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        if (editPwd && !isPwdValid()) {
            return;
        }

        isLoading = true;

        const data = {
            email: formValues.email,
            given_name: formValues.givenName,
            family_name: formValues.familyName,
        };
        if (editPwd) {
            data.password_current = pwdFormValues.current;
            data.password_new = pwdFormValues.new;
        }

        let res = await putUserSelf(user.id, data);
        if (res.ok) {
            success = true;
            user.email = formValues.email;
            user.given_name = formValues.givenName;
            user.family_name = formValues.familyName;
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

<div class="wrapper">
    <div class="container">
        <Input
                bind:value={formValues.email}
                bind:error={formErrors.email}
                autocomplete="family-name"
                placeholder="E-Mail"
                on:input={validateForm}
                width={inputWidth}
        >
            E-MAIL
        </Input>
        <Input
                bind:value={formValues.givenName}
                bind:error={formErrors.givenName}
                autocomplete="given-name"
                placeholder="Given Name"
                on:input={validateForm}
                width={inputWidth}
        >
            GIVEN NAME
        </Input>
        <Input
                bind:value={formValues.familyName}
                bind:error={formErrors.familyName}
                autocomplete="email"
                placeholder="Family Name"
                on:input={validateForm}
                width={inputWidth}
        >
            FAMILY NAME
        </Input>

        {#if editPwd}
            <div in:blur|global={{ duration: 350 }}>
                <AccModPwd
                        bind:formValues={pwdFormValues}
                        bind:isValid={isPwdValid}
                        btnWidth={btnWidth}
                        inputWidth={inputWidth}
                />
            </div>
        {/if}

        <Button width={btnWidth} bind:selected={editPwd}>
            CHANGE PASSWORD
        </Button>

        <Button width={btnWidth} on:click={onSubmit} level={1}>
            SAVE
        </Button>

        <div class="bottom">
            {#if success}
                <div class="success" transition:fade|global>
                    Update successful
                </div>
            {:else if err}
                <div class="err" transition:fade|global>
                    {err}
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .wrapper {
        display: flex;
        flex-direction: row;
    }

    .container {
        padding: 0 5px;
        display: flex;
        flex-direction: column;
    }

    .err {
        width: 230px;
        margin: -5px 10px 0 35px;
        padding-right: 5px;
        color: var(--col-err);
    }

    .err {
        margin: 5px;
        color: var(--col-err);
    }

    .bottom {
        height: 1em;
    }

    .success {
        margin: 5px;
        color: var(--col-ok);
    }
</style>
