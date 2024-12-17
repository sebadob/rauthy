<script>
    import {run} from 'svelte/legacy';

    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {getCerts, postRotateJwk} from "../../../../utils/dataFetchingAdmin.js";
    import JWK from "./JWK.svelte";

    let certs = $state([]);
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let timer = $state();

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                success = false;
            }, 3000);
        }
    });

    onMount(() => {
        if (certs.length === 0) {
            fetchCerts();
        }
    });

    async function fetchCerts() {
        let res = await getCerts();
        let body = await res.json();
        if (!res.ok) {
            err = body.message;
        } else {
            certs = [...body.keys];
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        let res = await postRotateJwk();
        if (res.ok) {
            fetchCerts();
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
        <h3>Json Web Keys</h3>
        <p>
            These are the Json Web Keys (JWKs) used for token singing.
        </p>

        <p>
            You can rotate them and generate a full new set. Depending on your deployment, this could take a few
            seconds.<br>
            New tokens will always be signed with the new / latest ones. The old keys will be cleaned up automatically,
            when
            there cannot be a token anymore that used the old key to not
            break any current token validation.
        </p>
    </div>

    {#each certs as jwk, i (jwk.kid)}
        <JWK bind:jwk={certs[i]}/>
    {/each}

    <!-- Save Button -->
    <Button on:click={onSubmit} bind:isLoading level={1}>ROTATE KEYS</Button>

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

<style>
    .desc {
        margin: 20px 5px 10px 5px;
        max-width: 600px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 5px;
    }

    .success {
        color: var(--col-ok);
    }

    .wrapper {
        margin: 0 5px;
    }
</style>
