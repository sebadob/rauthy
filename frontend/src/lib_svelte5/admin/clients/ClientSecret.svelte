<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {fetchPost, fetchPut, type IResponse} from "$api/fetch.ts";
    import type {ClientResponse, ClientSecretResponse} from "$api/types/clients.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import InputPassword from "$lib5/form/InputPassword.svelte";

    let {
        client,
    }: {
        client: ClientResponse,
    } = $props();

    let ta = useI18nAdmin();

    let err = $state('');
    let secret: undefined | string = $state();

    $effect(() => {
        err = '';
        secret = '';
        if (client.confidential) {
            fetchSecret();
        } else {
            err = ta.clients.confidentialNoSecret;
        }
    });

    async function fetchSecret() {
        let res = await fetchPost<ClientSecretResponse>(`/auth/v1/clients/${client.id}/secret`);
        handleResponse(res);
    }

    async function generateSecret() {
        let res = await fetchPut<ClientSecretResponse>(`/auth/v1/clients/${client.id}/secret`);
        handleResponse(res);
    }

    function handleResponse(res: IResponse<ClientSecretResponse>) {
        if (res.body) {
            if (res.body.secret) {
                secret = res.body.secret;
            }
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<div class="container">
    <div class="err">
        {err}
    </div>

    {#if secret}
        <InputPassword
                bind:value={secret}
                autocomplete="off"
                label="Client Secret"
                placeholder="Client Secret"
                disabled
                showCopy
        />

        <Button onclick={generateSecret}>
            {ta.clients.generateSecret}
        </Button>
    {/if}
</div>

<style>
    .container {
        margin: .5rem 0;
    }
</style>
