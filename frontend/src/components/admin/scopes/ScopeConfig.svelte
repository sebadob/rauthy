<script>
    import * as yup from "yup";
    import {extractFormErrors, isDefaultScope} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {REGEX_ROLES} from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {putScope} from "../../../utils/dataFetchingAdmin.js";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import Input from "$lib/inputs/Input.svelte";

    export let attrs;
    export let scope = {};
    export let onSave;

    let isLoading = false;
    let err = '';
    let success = false;
    let timer;
    let isDefault = false;
    let allAttrs = [];

    $: if (success) {
        timer = setTimeout(() => {
            success = false;
            onSave();
        }, 2000);
    }

    $: if (attrs) {
        allAttrs = attrs.map(a => a.name);
    }

    onMount(() => {
        isDefault = isDefaultScope(scope.name);
        return () => clearTimeout(timer);
    });

    let formErrors = {};
    const schema = yup.object().shape({
        name: yup.string().trim().matches(REGEX_ROLES, "Can only contain: 'a-z0-9-_/', length: 2-128"),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        const valid = await validateForm();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        let req = {
            scope: scope.name
        }
        if (scope.attr_include_access.length > 0) {
            req.attr_include_access = scope.attr_include_access;
        }
        if (scope.attr_include_id.length > 0) {
            req.attr_include_id = scope.attr_include_id;
        }

        let res = await putScope(scope.id, req);
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
            await schema.validate(scope, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

</script>

<div class="container">
    <div class="unit">
        <div class="label font-label">
            ID
        </div>
        <div class="value font-mono">
            {scope.id}
        </div>
    </div>

    <Input
            bind:value={scope.name}
            bind:error={formErrors.name}
            autocomplete="off"
            placeholder="Scope Name"
            on:input={validateForm}
            disabled={isDefault}
    >
        SCOPE NAME
    </Input>

    <!-- Mappings -->
    <div class="separator"></div>
    {#if isDefault}
        <div class="desc">
            <p>
                Custom mappings cannot be done for OpenID default scopes and their names cannot be changed.
            </p>
        </div>
    {:else}
        <div class="desc">
            <p>
                You can map custom scopes to attributes.<br>
                All additional attributes, that were configured, can have a custom value for each user.<br>
                When they are mapped to a scope, they can be included in the Access and / or ID Tokens.
            </p>
        </div>

        <!-- Access Mappings -->
        <div class="unit" style:margin-top="-3px">
            <div class="label">
                ACCESS TOKEN MAPPINGS
            </div>
            <ItemTiles
                    options={allAttrs}
                    bind:items={scope.attr_include_access}
                    searchThreshold={4}
            />
        </div>

        <!-- ID Mappings -->
        <div class="unit" style:margin-top="-3px">
            <div class="label">
                ID TOKEN MAPPINGS
            </div>
            <ItemTiles
                    options={allAttrs}
                    bind:items={scope.attr_include_id}
                    searchThreshold={4}
            />
        </div>
    {/if}

    <!-- Save Button-->
    {#if !isDefault}
        <Button on:click={onSubmit} level={1}>SAVE</Button>

        {#if success}
            <div class="success">
                Success
            </div>
        {/if}

        {#if err}
            <div class="mainErr err">
                {err}
            </div>
        {/if}
    {/if}
</div>

<style>
    .container {
        padding: 0 10px 10px 10px;
    }

    .desc {
        margin: 0 5px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .success {
        margin: 0 5px;
    }

    .label {
        margin: 5px 5px 0 5px;
        font-size: .9em;
    }

    .success {
        color: var(--col-ok);
    }

    .unit {
        margin: 7px 0;
    }

    .value {
        margin-left: 5px;
        display: flex;
    }
</style>
