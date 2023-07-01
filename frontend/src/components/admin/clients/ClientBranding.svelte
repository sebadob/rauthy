<script>
	import Button from "$lib/Button.svelte";
	import {
		deleteClientColors, deleteClientLogo,
		getClientColors,
		putClientColors,
		putClientLogo
	} from "../../../utils/dataFetchingAdmin.js";
	import { onDestroy, onMount } from "svelte";
	import ClientBrandingPreview from "./ClientBrandingPreview.svelte";
	import Input from "$lib/inputs/Input.svelte";
	import { extractFormErrors } from "../../../utils/helpers.js";
	import * as yup from "yup";
	import { REGEX_CSS_COLOR } from "../../../utils/constants.js";
	import ImageUpload from "../../ImageUpload.svelte";
	import { getClientLogo } from "../../../utils/dataFetching.js";

	export let client = {};
	let colors;
	let clientLogo;

	let isLoading = false;
	let err = '';
	let success = true;
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

		let res = await putClientColors(client.id, colors);
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
			await schema.validate(colors, {abortEarly: false});
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
  <div>
    <div class="label">
      <p>
        You can set client specific colors, which then will be used for the Login page.<br>
        These colors must be valid CSS values.
      </p>
    </div>

    {#if formValues}
      <div class="colors">
        <div class="col1">
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
            <div class="colorBlock" style:background={colors?.act1}></div>
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
            <div class="colorBlock" style:background={colors?.act1a}></div>
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
            <div class="colorBlock" style:background={colors?.act2}></div>
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
            <div class="colorBlock" style:background={colors?.act2a}></div>
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
            <div class="colorBlock" style:background={colors?.acnt}></div>
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
            <div class="colorBlock" style:background={colors?.acnta}></div>
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
            <div class="colorBlock" style:background={colors?.ok}></div>
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
            <div class="colorBlock" style:background={colors?.err}></div>
          </div>
        </div>

        <div class="col2">
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
              <div class="colorBlock" style:background={colors?.glow}></div>
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
            <div class="colorBlock" style:background={colors?.gmid}></div>
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
            <div class="colorBlock" style:background={colors?.ghigh}></div>
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
            <div class="colorBlock" style:background={colors?.text}></div>
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
            <div class="colorBlock" style:background={colors?.bg}></div>
          </div>

          <div class="upload">
            <ImageUpload bind:image={clientLogo} />
            {#if clientLogo}
              <img class="logo" src="{clientLogo}" alt="Custom Logo" />
            {/if}
          </div>
        </div>
      </div>
    {/if}

    <Button on:click={onSubmit} bind:isLoading level={1}>
      SAVE
    </Button>
    <Button on:click={onReset} bind:isLoading level={3}>
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

  {#if colors}
    <div class="preview">
      <ClientBrandingPreview bind:colors bind:clientLogo />
    </div>
  {/if}
</div>

<style>
    .col1 {
    }

    .col2 {
        margin: 0 30px;
    }

    .colorBlock {
        margin: 13px 0 0 -38px;
        width: 30px;
        height: 31px;
        border-radius: 0 5px 5px 0;
        z-index: 1;
    }

    .colors {
        display: flex;
    }

    .container {
        margin: 0 10px 20px 10px;
        display: flex;
    }

    .err, .success {
        margin: 0 5px;
    }

    .err {
        display: flex;
        align-items: center;
        color: var(--col-err);
    }

    .label {
        margin: 5px 7px 5px 7px;
    }

    .logo {
        margin-top: 22px;
        width: 84px;
        height: 84px;
    }

    .preview {
        margin: 17px 10px 0 40px;
    }

    .row {
        display: flex;
    }

    .success {
        color: var(--col-ok);
    }

    .upload {
        margin: 15px 5px;
    }
</style>
