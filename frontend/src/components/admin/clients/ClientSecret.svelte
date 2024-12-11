<script>
    import {onMount} from "svelte";
    import {getClientSecret, putClientSecret} from "../../../utils/dataFetchingAdmin.js";
    import Button from "$lib/Button.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";

    let { client } = $props();

    let err = $state('');
    let secret = $state('');

    onMount(() => {
        fetchSecret();
    });

    async function fetchSecret() {
        let res = await getClientSecret(client.id);

        let body = await res.json();
        if (!res.ok) {
            err = body.message;
        } else {
            secret = body.secret;
        }
    }

    async function generateSecret() {
        let res = await putClientSecret(client.id);

        let body = await res.json();
        if (!res.ok) {
            err = body.message;
        } else {
            await fetchSecret();
        }
    }
</script>

<div class="container">
    <div class="err">
        {err}
    </div>

    <div class="value font-mono">
        {#if secret}
            <PasswordInput
                    bind:value={secret}
                    autocomplete="off"
                    disabled
                    showCopy
                    width="inherit"
                    maxWidth="800px"
            >
                CLIENT SECRET
            </PasswordInput>
        {/if}
    </div>

    <Button
            on:click={generateSecret}
            level={1}
            width={130}
    >
        GENERATE NEW
    </Button>
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
</style>
