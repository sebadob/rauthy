<script lang="ts">
    import { onMount } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import Section from '#lib/Section.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import IconCheck from '#icons/IconCheck.svelte';
    import { fetchGet, fetchPost } from '#api/fetch.ts';
    import type { EncKeyMigrateRequest, EncKeysResponse } from '#api/types/enc_keys.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import Options from '#lib/Options.svelte';

    let ta = useI18nAdmin();
    const t = useI18n();

    let err = $state('');
    let isLoading = $state(true);
    let activeKey = $state('');
    let migrateKey = $state('');
    let keys: string[] = $state([]);
    let success = $state(false);

    onMount(async () => {
        let res = await fetchGet<EncKeysResponse>('/auth/v1/encryption/keys');
        if (res.body) {
            keys = res.body.keys;
            activeKey = res.body.active;
            migrateKey = res.body.active;
        } else {
            err = res.error?.message || 'Error';
        }
        isLoading = false;
    });

    async function migrate() {
        isLoading = true;

        let payload: EncKeyMigrateRequest = {
            key_id: migrateKey,
        };
        let res = await fetchPost('/auth/v1/encryption/migrate', payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 4000);
        }

        isLoading = false;
    }
</script>

<h2 class="ui-page-title">{ta.docs.encKeys.header}</h2>

<div class="ui-section-stack">
    <Section title={ta.docs.encKeys.keysAvailable}>
        {#if activeKey}
            <p class="current">
                {ta.docs.encKeys.keyActive}: <strong class="font-mono">{activeKey}</strong>
            </p>
            <ul class="keys" role="list">
                {#each keys as key (key)}
                    <li class="font-mono" class:active={key === activeKey}>{key}</li>
                {/each}
            </ul>
        {/if}
        <details>
            <summary>{t.common.details}</summary>
            <p class="ui-muted-copy">{ta.docs.encKeys.p1}</p>
            <p class="ui-muted-copy">{ta.docs.encKeys.p2}</p>
        </details>
    </Section>

    <Section title={ta.docs.encKeys.migrate}>
        <p class="ui-muted-copy">{ta.docs.encKeys.p3}</p>
        {#if keys.length > 1}
            <div class="controls">
                <div class="admin-field-select target">
                    <span class="admin-field-label">{ta.docs.encKeys.migrateToKey}</span>
                    <Options
                        ariaLabel={ta.docs.encKeys.migrateToKey}
                        options={keys}
                        bind:value={migrateKey}
                    />
                </div>
                <Button onclick={migrate} {isLoading}>{ta.docs.encKeys.migrate}</Button>
                {#if success}<IconCheck />{/if}
            </div>
        {:else if !isLoading && !err}
            <p class="ui-muted-copy">{ta.docs.encKeys.pNotPossible}</p>
        {/if}
        {#if err}<p class="err" role="alert">{err}</p>{/if}
    </Section>
</div>

<style>
    .keys,
    .controls {
        display: flex;
        flex-wrap: wrap;
        gap: 0.625rem;
    }

    .keys {
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .keys li {
        min-width: 5rem;
        padding: 0.625rem 0.875rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-container);
        font-size: var(--ui-control-font-size);
        overflow-wrap: anywhere;
    }

    .keys li.active {
        color: var(--ui-primary);
        background: var(--ui-tonal);
        border-color: var(--ui-primary);
    }

    .controls {
        align-items: end;
    }

    .target {
        width: min(100%, 24rem);
    }

    p {
        margin: 0;
    }

    details {
        padding: 0.625rem 0.75rem;
        border-radius: var(--border-radius);
        background: var(--ui-container);
    }

    summary {
        cursor: pointer;
        font-size: var(--ui-control-font-size);
    }

    details p {
        margin-top: 0.5rem;
    }
</style>
