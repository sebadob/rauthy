<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import Section from '#lib/Section.svelte';
    import { slide } from 'svelte/transition';
    import HiddenValueArea from '#lib/HiddenValueArea.svelte';
    import type { ApiKeyResponse } from '#api/types/api_keys.ts';
    import { fetchPut } from '#api/fetch.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import InputPassword from '#lib/form/InputPassword.svelte';

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
    let generating = $state(false);

    $effect(() => {
        if (key.name) {
            secret = '';
            err = '';
        }
    });

    async function generateSecret() {
        err = '';
        generating = true;
        let res = await fetchPut<string>(`/auth/v1/api_keys/${key.name}/secret`);

        if (res.text) {
            secret = res.text;
            curl = `curl -s -H 'Authorization: API-Key ${res.text}' ${window.location.origin}/auth/v1/api_keys/${key.name}/test`;
            curlPretty = `${curl} | jq`;
        } else {
            err = res.error?.message || 'Error';
        }
        generating = false;
    }
</script>

<div class="sections">
    <Section title={ta.api_key.secret ?? 'Secret'}>
        {#snippet actions()}
            <Button level={!secret ? 1 : 2} onclick={generateSecret} isLoading={generating}>
                {t.passwordReset.generate}
            </Button>
        {/snippet}
        <p class="ui-muted-copy">{ta.api_key.generate1}</p>
        <p class="warning">{ta.api_key.generate2}</p>
        {#if secret}
            <div class="secret" transition:slide={{ duration: 150 }}>
                <InputPassword
                    autocomplete="off"
                    value={secret}
                    label="API Key"
                    placeholder="API Key"
                    disabled
                    showCopy
                />
            </div>
        {/if}
        {#if err}<p class="err" role="alert">{err}</p>{/if}
    </Section>

    {#if secret}
        <Section title={ta.api_key.examples ?? 'Usage examples'}>
            <p class="ui-muted-copy">{@html ta.api_key.generate3}</p>
            <div><code>{'API-Key <api_key>'}</code></div>
            <div class="example">
                <p class="ui-muted-copy">{@html ta.api_key.generate4}</p>
                {#key curlPretty}
                    <HiddenValueArea
                        ariaLabel="curl text command with jq"
                        rows={3}
                        value={curlPretty}
                    />
                {/key}
            </div>
            <div class="example">
                <p class="ui-muted-copy">{@html ta.api_key.generate5}</p>
                {#key curl}
                    <HiddenValueArea ariaLabel="curl text command simple" rows={3} value={curl} />
                {/key}
            </div>
        </Section>
    {/if}
</div>

<style>
    .sections,
    .example {
        display: grid;
        gap: var(--ui-space);
    }

    .example {
        gap: 0.5rem;
    }

    .secret {
        max-width: 30rem;
    }

    .warning {
        max-width: 70ch;
        padding: 0.625rem 0.75rem;
        border-inline-start: 2px solid hsl(var(--error) / 0.55);
        background: var(--ui-hover);
        font-size: var(--ui-control-font-size);
    }
</style>
