<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { slide } from 'svelte/transition';
    import HiddenValueArea from '$lib5/HiddenValueArea.svelte';
    import type { ApiKeyResponse } from '$api/types/api_keys.ts';
    import { fetchPut } from '$api/fetch';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import InputPassword from '$lib5/form/InputPassword.svelte';

    let {
        key,
    }: {
        key: ApiKeyResponse;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let secret = $state('');
    let curl = $state('');
    let curlPretty = $state('');

    $effect(() => {
        if (key.name) {
            secret = '';
        }
    });

    async function generateSecret() {
        let res = await fetchPut<string>(`/auth/v1/api_keys/${key.name}/secret`);

        if (res.text) {
            secret = res.text;
            curl = `curl -s -H 'Authorization: API-Key ${res.text}' ${window.location.origin}/auth/v1/api_keys/${key.name}/test`;
            curlPretty = `${curl} | jq`;
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<p>{ta.api_key.generate1}</p>
<p>{ta.api_key.generate2}</p>

<div class="btn">
    <Button
        level={!secret ? 1 : 3}
        onclick={generateSecret}
    >
        {t.passwordReset.generate}
    </Button>
</div>

{#if secret}
    <div
        transition:slide={{ duration: 150 }}
        class="secret font-mono"
    >
        <InputPassword
            autocomplete="off"
            value={secret}
            label="API Key"
            placeholder="API Key"
            disabled
            showCopy
            width="min(25rem, calc(100dvw - .5rem))"
        />
    </div>

    <div transition:slide={{ duration: 150 }}>
        <p>{@html ta.api_key.generate3}</p>
        <p>
            <span class="headerCode"><b><code>{'API-Key <api_key>'}</code></b></span>
        </p>
    </div>

    <div transition:slide={{ duration: 150 }}>
        <p>{@html ta.api_key.generate4}</p>
        {#key curlPretty}
            <HiddenValueArea
                ariaLabel="curl text command with jq"
                rows={5}
                value={curlPretty}
            />
        {/key}
    </div>

    <div transition:slide={{ duration: 150 }}>
        <p>{@html ta.api_key.generate5}</p>
        {#key curlPretty}
            <HiddenValueArea
                ariaLabel="curl text command simple"
                rows={5}
                value={curl}
            />
        {/key}
    </div>
{/if}

<div class="err">
    {err}
</div>

<style>
    .btn {
        margin: 1rem 0;
    }

    .secret {
        margin-top: 1rem;
        max-width: 40rem;
    }
</style>
