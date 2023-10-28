<script>
    import {putApiKeySecret} from "../../../utils/dataFetchingAdmin.js";
    import Button from "$lib/Button.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import {slide} from "svelte/transition";
    import HiddenValueArea from "$lib/inputs/HiddenValueArea.svelte";

    export let apiKey;

    let err = '';
    let secret = '';
    let curl = '';
    let curlPretty = '';

    async function generateSecret() {
        let res = await putApiKeySecret(apiKey.name);

        if (res.ok) {
            secret = await res.text();
            curl = `curl -H 'Authorization: API-Key ${secret}' ${window.location.origin}/auth/v1/api_keys/${apiKey.name}/test`;
            curlPretty = `${curl} | jq`;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }
</script>

<div class="container">
    <p>You Can generate a new secret for this API Key here.</p>
    <p>
        You will only see this secret once after the generation.
        When a new one has been generated, the old secret will be overridden permanently.
        This operation cannot be reverted!
    </p>

    <Button
            on:click={generateSecret}
            level={!secret ? 1 : 3}
            width={130}
    >
        GENERATE NEW
    </Button>

    {#if secret}
        <div transition:slide class="secret font-mono">
            <PasswordInput
                    bind:value={secret}
                    autocomplete="off"
                    disabled
                    showCopy
                    width="inherit"
                    maxWidth="800px"
            >
                API KEY
            </PasswordInput>
        </div>

        <p>
            An API Key must be provided in the HTTP <span class="headerCodeSmall"><code>Authorization</code></span>
            header in the following format:
        </p>
        <p><span class="headerCode"><b><code>API-Key YourSecretApiKeyHere</code></b></span></p>

        <p>You can use the following <code>curl</code> request to test your new Key:</p>
        <HiddenValueArea
                name="id"
                rows={3}
                value={curlPretty}
        />

        <p>If you don't have <code>jq</code> installed and the above fails:</p>
        <HiddenValueArea
                name="id"
                rows={3}
                value={curl}
        />
    {/if}

    <div class="err">
        {err}
    </div>

</div>

<style>
    .container {
        margin: 0 10px 20px 10px;
    }

    .container > p {
        margin: 1rem .5rem;
        max-width: 27rem;
    }

    .err {
        display: flex;
        align-items: center;
        margin: 0 10px;
        color: var(--col-err);
    }

    .headerCode {
        font-size: 1.1rem;
    }

    .headerCodeSmall {
        font-size: 1rem;
    }

    .secret {
        margin-top: 1rem;
        max-width: 40rem;
    }
</style>
