<script>
    import Button from "$lib/Button.svelte";
    import {deleteApiKey} from "../../../utils/dataFetchingAdmin.js";

    export let apiKey = {};
    export let onSave;

    let err = '';
    let success = false;

    async function onSubmit() {
        err = '';

        let res = await deleteApiKey(apiKey.name);
        if (res.ok) {
            onSave();
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

</script>

<div class="container">
    <div class="label">
        Are you sure, you want to delete this ApiKey?
    </div>

    <Button on:click={onSubmit} level={1}>
        DELETE
    </Button>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0 10px 20px 10px;
    }

    .err {
        display: flex;
        align-items: center;
        margin: 0 10px;
        color: var(--col-err);
    }

    .label {
        margin: 5px 7px 5px 7px;
    }
</style>

