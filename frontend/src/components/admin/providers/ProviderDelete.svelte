<script>
    import Button from "$lib/Button.svelte";
    import {deleteProvider, getProviderDeleteSafe} from "../../../utils/dataFetchingAdmin";
    import {onMount} from "svelte";
    import Switch from "$lib/Switch.svelte";

    export let provider = {};
    export let onSave;

    let isLoading = true;
    let err = '';
    let forceDelete = false;
    let linkedUsers = [];

    $: console.log(linkedUsers);

    onMount(async () => {
        let res = await getProviderDeleteSafe(provider.id);
        let body = await res.json();

        // if there are any linked users, the backend will return 406
        // instead of 200 to make it clear, that it is not safe to delete
        if (res.status === 406) {
            linkedUsers = body;
        } else {
            err = body.message;
        }

        isLoading = false;
    })

    async function onSubmit() {
        err = '';
        isLoading = true;

        let res = await deleteProvider(provider.id);
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
    {#if linkedUsers.length > 0}
        <div class="label">
            <p>This provider is in use by active users.</p>
            <p>
                You can force delete it, but users without a local password or passkey
                will not be able to log in anymore.
            </p>

            <details>
                <summary>Linked Users</summary>
                {#each linkedUsers as user (user.id)}
                    <p>{user.id} {user.email}</p>
                {/each}
            </details>
        </div>

        <div class="label">
            <p>
                Are you sure you want to delete this provider?
            </p>
            <div class="forceDelete">
                FORCE DELETE
                <Switch bind:selected={forceDelete}/>
            </div>
        </div>

        {#if forceDelete}
            <Button on:click={onSubmit} bind:isLoading level={1}>
                DELETE
            </Button>
        {/if}
    {:else}
        <div class="label">
            Are you sure, you want to delete this provider?
        </div>

        <Button on:click={onSubmit} bind:isLoading level={1}>
            DELETE
        </Button>
    {/if}
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

    .forceDelete {
        color: var(--col-err);
        font-weight: bold;
    }

    .label {
        margin: 5px 7px 5px 7px;
    }
</style>
