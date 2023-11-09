<script>
    import Button from "$lib/Button.svelte";
    import {fade, slide} from 'svelte/transition';
    import {putUserWebIdData} from "../../utils/dataFetching.js";
    import Switch from "$lib/Switch.svelte";
    import {REGEX_URI} from "../../utils/constants.js";
    import AccWebIdEntries from "./AccWebIdEntries.svelte";

    export let t;
    export let webIdData;
    export let viewModePhone;

    const btnWidth = "12rem";

    let err = '';
    let success = false;
    let successEmailConfirm = false;

    $: webIdLink = webIdData.is_open ?
        `${window.location.origin}/auth/v1/users/${webIdData.user_id}/webid`
        : '';

    $: if (success) {
        setTimeout(() => {
            success = false;
        }, 3000);
    }

    // let formValues = {
    //     email: user.email,
    //     givenName: user.given_name,
    //     familyName: user.family_name,
    // };
    // let formErrors = {};
    //
    // const schema = yup.object().shape({
    //     email: yup.string()
    //         .required(t.validEmail)
    //         .email(t.validEmail),
    //     givenName: yup.string()
    //         .required(t.validGivenName)
    //         .matches(REGEX_NAME, t.validGivenName),
    //     familyName: yup.string()
    //         .required(t.validFamilyName)
    //         .matches(REGEX_NAME, t.validFamilyName),
    // });

    // function handleKeyPress(event) {
    //     if (event.code === 'Enter') {
    //         onSubmit();
    //     }
    // }

    async function onSubmit() {
        // const valid = await validateForm();
        // if (!valid) {
        //     err = t.invalidInput;
        //     return;
        // }

        const data = {
            user_id: webIdData.user_id,
            is_open: webIdData.is_open,
        };

        console.log(data);

        // TODO add key / value pairs

        let res = await putUserWebIdData(data.user_id, data);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

    // async function validateForm() {
    //     try {
    //         await schema.validate(formValues, {abortEarly: false});
    //         formErrors = {};
    //         return true;
    //     } catch (err) {
    //         formErrors = extractFormErrors(err);
    //         return false;
    //     }
    // }

</script>

<div class="wrapper">
    <div class="container">
        <p>{t.webIdDesc}</p>

        <div class="row">
            <div class="label">
                {t.webIdActive}
            </div>
            <Switch bind:selected={webIdData.is_open}/>
        </div>

        {#if webIdData.is_open}
            <div transition:slide>
                <p>{t.webIdLinkText}</p>
                <p><a href={webIdLink} target="_blank">{webIdLink}</a></p>

                <!-- Custom Data -->
                <p>{t.webIdDescData}</p>
                <AccWebIdEntries bind:t bind:webIdData bind:viewModePhone />
            </div>
        {/if}

        <Button width={btnWidth} on:click={onSubmit} level={1}>
            {t.save.toUpperCase()}
        </Button>

        <div class="bottom">
            {#if success}
                <div class="success" transition:fade>
                    Update successful
                </div>
            {:else if err}
                <div class="err" transition:fade>
                    {err}
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    p {
        margin: 0 .5rem .5rem .5rem;
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

    .row {
        margin: .5rem;
        display: flex;
        gap: .5rem;
    }

    .success {
        margin: 5px;
        color: var(--col-ok);
    }

    .wrapper {
        margin-top: 10px;
        display: flex;
        flex-direction: row;
    }
</style>
