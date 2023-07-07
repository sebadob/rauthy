<script>
    import Button from "$lib/Button.svelte";
    import {deleteSessionsForUser} from "../../../utils/dataFetchingAdmin.js";

    export let user = {};

    let isLoading = false;
    let err = '';
    let success = false;

    async function onSubmit() {
        err = '';
        isLoading = true;

        let res = await deleteSessionsForUser(user.id);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }
</script>

<div class="container">
    <div class="label">
        Invalidate all existing sessions and refresh tokens for this user?
    </div>

    <Button on:click={onSubmit} bind:isLoading level={1} width="5rem">
        LOGOUT
    </Button>

    {#if err}
        <div class="err">
            {err}
        </div>
    {:else if success}
        <div class="success">
            Success
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0 10px 20px 10px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 5px;
    }

    .label {
        margin: 5px 7px 5px 7px;
    }

    .success {
        color: var(--col-ok);
    }
</style>
