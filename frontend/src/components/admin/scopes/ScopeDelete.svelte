<script>
    import Button from "$lib/Button.svelte";
    import {deleteScope} from "../../../utils/dataFetchingAdmin";
    import {isDefaultScope} from "../../../utils/helpers";

    let {scope = {}, onSave} = $props();

    let isLoading = $state(false);
    let err = $state('');
    let success = false;

    async function onSubmit() {
        err = '';
        isLoading = true;

        let res = await deleteScope(scope.id);
        if (res.ok) {
            onSave();
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

</script>

<div class="container">
    {#if isDefaultScope(scope.name)}
        <div class="label">
            OpenID default scopes cannot be deleted
        </div>
    {:else}
        <div class="label">
            Are you sure, you want to delete this scope?
        </div>

        <Button on:click={onSubmit} bind:isLoading level={1}>
            DELETE
        </Button>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
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
