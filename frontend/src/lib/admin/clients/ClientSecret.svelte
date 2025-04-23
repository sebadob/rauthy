<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {fetchPost, fetchPut} from "$api/fetch";
    import type {ClientResponse, ClientSecretRequest, ClientSecretResponse} from "$api/types/clients.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import {slide} from "svelte/transition";
    import InputCheckbox from "$lib/form/InputCheckbox.svelte";
    import Input from "$lib/form/Input.svelte";
    import IconCheck from "$icons/IconCheck.svelte";

    let {
        client,
    }: {
        client: ClientResponse,
    } = $props();

    let ta = useI18nAdmin();

    let err = $state('');
    let isInputErr = $state(false);
    let success = $state(false);

    let secret: undefined | string = $state();
    let showConfirm = $state(false);
    let cacheSecret = $state(false);
    let cacheCurrentHours = $state('1');

    $effect(() => {
        if (client.id) {
            showConfirm = false;
            cacheSecret = false;
            cacheCurrentHours = '1';

            err = '';
            secret = '';
            if (client.confidential) {
                fetchSecret();
            } else {
                err = ta.clients.confidentialNoSecret;
            }
        }
    });

    async function fetchSecret() {
        let res = await fetchPost<ClientSecretResponse>(`/auth/v1/clients/${client.id}/secret`);
        if (res.body) {
            if (res.body.secret) {
                secret = res.body.secret;
            }
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function generateSecret() {
        let payload: ClientSecretRequest = {
            cache_current_hours: cacheSecret ? Number.parseInt(cacheCurrentHours) : undefined,
        };
        let res = await fetchPut<ClientSecretResponse>(`/auth/v1/clients/${client.id}/secret`, payload);
        if (res.body) {
            if (res.body.secret) {
                secret = res.body.secret;
                success = true;
                setTimeout(() => {
                    success = false;
                }, 3000);
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

        {#if showConfirm}
            <div transition:slide={{ duration: 150 }}>
                <p>{ta.clients.secret.rotateDesc1}</p>
                <p><b>{ta.clients.secret.rotateDesc2}</b></p>

                <InputCheckbox
                        ariaLabel="Client Secret Cache"
                        bind:checked={cacheSecret}
                >
                    {ta.clients.secret.doCache}
                </InputCheckbox>

                {#if cacheSecret}
                    <div transition:slide={{ duration: 150 }}>
                        <Input
                                typ="number"
                                label={ta.clients.secret.cacheDuration}
                                placeholder={ta.clients.secret.cacheDuration}
                                bind:value={cacheCurrentHours}
                                bind:isError={isInputErr}
                                min="1"
                                max="24"
                                width="12rem"
                        />
                    </div>
                {/if}

                <div class="flex gap-05" style:margin-top="1rem">
                    <Button onclick={generateSecret} isDisabled={isInputErr}>
                        {ta.clients.secret.generate}
                    </Button>
                    {#if success}
                        <IconCheck/>
                    {/if}
                </div>
            </div>
        {:else}
            <Button onclick={() => showConfirm = true}>
                {ta.clients.secret.generate}
            </Button>
        {/if}
    {/if}
</div>

<style>
    .container {
        margin: .5rem 0;
        /* matches <p> max width */
        max-width: 467pt;
    }
</style>
