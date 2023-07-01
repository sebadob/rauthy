<script>
    import {onMount} from "svelte";
    import {getEncKeys, postEncMigrate} from "../../../../utils/dataFetchingAdmin.js";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import Button from "$lib/Button.svelte";

    let err = '';
    let isLoading = true;
    let activeKey = '';
    let migrateKey = '';
    let keys = [];
    let success = false;

    onMount(async () => {
        fetchData();
    });

    async function fetchData() {
        let res = await getEncKeys();
        let body = await res.json();
        if (res.ok) {
            activeKey = body.active;
            migrateKey = body.active;
            keys = body.keys;
        } else {
            err = body.message;
        }
        isLoading = false;
    }

    async function migrate() {
        isLoading = true;
        success = false;

        let data = {
            key_id: migrateKey,
        };

        let res = await postEncMigrate(data);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }
</script>

<div class="wrapper">
    <div class="desc">
        <h3>Secrets Encryption Keys</h3>

        <p>
            These Keys are used for an additional encryption at rest, independently from any data store technology used
            under
            the hood. They are configured statically, but can be rotated and migrated on this page manually to maybe get
            rid
            of an old key, which is currently still in use in some places.
        </p>

        <p>
            The active key is statically set in the Rauthy config file / environment variables. It cannot be changed
            here
            dynamically.<br>
            All new encryptions will always use the currently active key.
        </p>

        <div class="valRow">
            <div class="label">
                Active Key:
            </div>

            <div class="value font-mono">
                {activeKey}
            </div>
        </div>

        <div class="keysBlock">
            <div>
                Available Keys:
            </div>

            <ul style:margin-left="-10px">
                {#each keys as key}
                    <li class="font-mono">
                        {key}
                    </li>
                {/each}
            </ul>
        </div>

        <div class="migrateBlock">
            <p>
                If you migrate all existing secrets, it might take a few seconds to finish, if you have a big
                dataset.<br>
                Manually migrate all existing encryption in the backend to key:
            </p>

            <OptionSelect bind:value={migrateKey} options={keys} width="186px"/>
        </div>
    </div>

    <div class="btn">
        <Button on:click={migrate} bind:isLoading>Migrate</Button>
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}

    {#if success}
        <div class="success">
            Success
        </div>
    {/if}
</div>

<style>
    .btn {
        margin: 10px 0;
    }

    .desc {
        margin: 0 5px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 5px;
    }

    .label {
        width: 90px;
    }

    .keysBlock {
        margin-top: 20px;
    }

    .migrateBlock {
        margin-top: 20px;
    }

    .success {
        color: var(--col-ok);
    }

    .valRow {
        display: flex;
    }

    .wrapper {
        margin: 0 5px;
    }
</style>
