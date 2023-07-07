<script>
    import Button from "$lib/Button.svelte";
    import {
        deleteClientColors,
        deleteClientLogo,
        getClientColors,
        putClientColors,
        putClientLogo
    } from "../../../utils/dataFetchingAdmin.js";
    import {onDestroy, onMount} from "svelte";
    import ClientBrandingPreview from "./ClientBrandingPreview.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import * as yup from "yup";
    import {REGEX_CSS_COLOR} from "../../../utils/constants.js";
    import ImageUpload from "../../ImageUpload.svelte";
    import {getClientLogo} from "../../../utils/dataFetching.js";

    export let client = {};
    let colors;
    let clientLogo;

    let isLoading = false;
    let err = '';
    let success = false;
    let timer;

    let formErrors = {};
    let formValues = {};
    const schema = yup.object().shape({
        act1: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        act1a: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        act2: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        act2a: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        acnt: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        acnta: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        ok: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        err: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        glow: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        gmid: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        ghigh: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        text: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
        bg: yup.string().trim().matches(REGEX_CSS_COLOR, "only valid CSS color"),
    });

    $: if (success) {
        timer = setTimeout(() => {
            success = false;
        }, 3000);
    }

    onMount(async () => {
        await getColors();
        await getLogo();
    });

    onDestroy(() => {
        if (timer) {
            clearTimeout(timer);
        }
    });

    async function getColors() {
        let res = await getClientColors(client.id);
        if (res.ok) {
            let cols = await res.json();
            colors = cols;
            formValues = cols;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

    async function getLogo() {
        let res = await getClientLogo(client.id);
        if (res.ok) {
            clientLogo = await res.text();
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        let isValid = await validateForm();
        if (!isValid) {
            return;
        }

        let res = await putClientColors(client.id, formValues);
        if (res.ok) {
            if (clientLogo) {
                let resLogo = await putClientLogo(client.id, clientLogo);
                if (resLogo.ok) {
                    success = true;
                } else {
                    let body = await res.json();
                    err = `Logo upload: ${body.message}`;
                }
            } else {
                success = true;
            }
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    async function onReset() {
        err = '';
        isLoading = true;

        let res = await deleteClientColors(client.id);
        if (res.ok) {
            await deleteClientLogo(client.id);
            await getColors();
            await getLogo();
            success = true;
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
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }

        colors = formValues;
        return true;
    }

</script>

<div class="container">
    <div class="label">
        <p>
            You can set client specific colors, which then will be used for the Login page.
            These colors must be valid CSS values. Either enter them directly or use the color picker.
        </p>
    </div>

    <div class="inner">
        {#if formValues}
                <div class="col1 colors">
                    <div class="row">
                        <Input
                                bind:value={formValues.act1}
                                bind:error={formErrors.act1}
                                autocomplete="off"
                                placeholder="act1"
                                on:input={validateForm}
                        >
                            act1
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.act1} />
                            <div class="colorBlock" style:background={formValues.act1}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.act1a}
                                bind:error={formErrors.act1a}
                                autocomplete="off"
                                placeholder="act1a"
                                on:input={validateForm}
                        >
                            act1a
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.act1a} />
                            <div class="colorBlock" style:background={formValues.act1a}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.act2}
                                bind:error={formErrors.act2}
                                autocomplete="off"
                                placeholder="act2"
                                on:input={validateForm}
                        >
                            act2
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.act2} />
                            <div class="colorBlock" style:background={formValues.act2}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.act2a}
                                bind:error={formErrors.act2a}
                                autocomplete="off"
                                placeholder="act2a"
                                on:input={validateForm}
                        >
                            act2a
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.act2a} />
                            <div class="colorBlock" style:background={formValues.act2a}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.acnt}
                                bind:error={formErrors.acnt}
                                autocomplete="off"
                                placeholder="acnt"
                                on:input={validateForm}
                        >
                            acnt
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.acnt} />
                            <div class="colorBlock" style:background={formValues.acnt}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.acnta}
                                bind:error={formErrors.acnta}
                                autocomplete="off"
                                placeholder="acnta"
                                on:input={validateForm}
                        >
                            acnta
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.acnta} />
                            <div class="colorBlock" style:background={formValues.acnta}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.ok}
                                bind:error={formErrors.ok}
                                autocomplete="off"
                                placeholder="ok"
                                on:input={validateForm}
                        >
                            ok
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.ok} />
                            <div class="colorBlock" style:background={formValues.ok}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.err}
                                bind:error={formErrors.err}
                                autocomplete="off"
                                placeholder="err"
                                on:input={validateForm}
                        >
                            err
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.err} />
                            <div class="colorBlock" style:background={formValues.err}></div>
                        </div>
                    </div>
                </div>

                <div class="col2 colors">
                    <div class="row">
                        <Input
                                bind:value={formValues.glow}
                                bind:error={formErrors.glow}
                                autocomplete="off"
                                placeholder="glow"
                                on:input={validateForm}
                        >
                            glow
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.glow} />
                            <div class="colorBlock" style:background={formValues.glow}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.gmid}
                                bind:error={formErrors.gmid}
                                autocomplete="off"
                                placeholder="gmid"
                                on:input={validateForm}
                        >
                            gmid
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.gmid} />
                            <div class="colorBlock" style:background={formValues.gmid}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.ghigh}
                                bind:error={formErrors.ghigh}
                                autocomplete="off"
                                placeholder="ghigh"
                                on:input={validateForm}
                        >
                            ghigh
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.ghigh} />
                            <div class="colorBlock" style:background={formValues.ghigh}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.text}
                                bind:error={formErrors.text}
                                autocomplete="off"
                                placeholder="text"
                                on:input={validateForm}
                        >
                            text
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.text} />
                            <div class="colorBlock" style:background={formValues.text}></div>
                        </div>
                    </div>

                    <div class="row">
                        <Input
                                bind:value={formValues.bg}
                                bind:error={formErrors.bg}
                                autocomplete="off"
                                placeholder="bg"
                                on:input={validateForm}
                        >
                            bg
                        </Input>
                        <div class="colInputWrap">
                            <input type="color" class="colInput" bind:value={formValues.bg} />
                            <div class="colorBlock" style:background={formValues.bg}></div>
                        </div>
                    </div>

                    <div class="upload">
                        <ImageUpload bind:image={clientLogo}/>
                        {#if clientLogo}
                            <img class="logo" src="{clientLogo}" alt="Custom Logo"/>
                        {/if}
                    </div>
                </div>
        {/if}

        {#if colors}
            <div class="preview">
                <ClientBrandingPreview bind:colors bind:clientLogo/>
            </div>
        {/if}
    </div>

    <div class="btns">
        <Button on:click={onSubmit} bind:isLoading level={1} width="4rem">
            SAVE
        </Button>
        <Button on:click={onReset} bind:isLoading level={3} width="4rem">
            RESET
        </Button>

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
    </div>
</div>

<style>
    .btns {
        margin-top: 1rem;
    }

    .col1, .col2 {
        display: flex;
        flex-direction: column;
    }

    .colorBlock, .colInput {
        position: absolute;
        top: -.825rem;
        right: .45rem;
        width: 1.9rem;
        aspect-ratio: 1;
        border-radius: 0 5px 5px 0;
    }

    .colorBlock {
        z-index: 1;
        pointer-events: none;
    }

    .colInput {
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        background-color: transparent;
        border: none;
        cursor: pointer;
    }

    .colInputWrap {
        position: relative;
        margin: auto;
    }

    .colors {
        display: flex;
        margin: .5rem .33rem .5rem .15rem;
    }

    .container {
        margin: 0 10px 20px 10px;
        display: flex;
        flex-direction: column;
    }

    .err, .success {
        margin: 0 5px;
    }

    .err {
        display: flex;
        align-items: center;
        color: var(--col-err);
    }

    .inner {
        display: flex;
        flex-wrap: wrap;
    }

    .label {
        margin: 5px 7px 5px 7px;
    }

    .logo {
        margin-top: 1.45rem;
        width: 84px;
        height: 84px;
    }

    .preview {
        margin: .33rem;
    }

    .row {
        display: flex;
    }

    .success {
        color: var(--col-ok);
    }

    .upload {
        margin: .9rem .3rem;
    }
</style>
