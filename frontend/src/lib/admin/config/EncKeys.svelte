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
            <ul class="keys" role="list">
                {#each keys as key (key)}
                    <li>
                        <code>{key}</code>
                        {#if key === activeKey}
                            <span class="active-key">{ta.docs.encKeys.keyActive}</span>
                        {/if}
                    </li>
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
    .keys {
        list-style: none;
        margin: 0;
        padding: 0;
        border-block: 1px solid var(--ui-outline);
    }

    .keys li {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
        min-height: 2.75rem;
        margin: 0;
        padding-block: 0.375rem;
        font-size: var(--ui-control-font-size);
        overflow-wrap: anywhere;
    }

    .keys li + li {
        border-top: 1px solid var(--ui-outline);
    }

    .keys code {
        min-width: 0;
        padding: 0;
        background: none;
        font-size: inherit;
    }

    .active-key {
        padding: 0.125rem 0.5rem;
        border-radius: var(--border-radius);
        color: var(--ui-text);
        background: var(--ui-tonal);
        font-size: 0.75rem;
        font-weight: 600;
        text-align: end;
    }

    .controls {
        display: flex;
        flex-wrap: wrap;
        align-items: end;
        gap: 0.625rem;
        --ui-button-height: var(--ui-field-height);
        --ui-button-min-height: var(--ui-field-height);
    }

    .target {
        width: min(100%, 24rem);
    }

    p {
        margin: 0;
    }

    details {
        padding-block: 0.25rem;
    }

    summary {
        cursor: pointer;
        font-size: var(--ui-control-font-size);
    }

    details p {
        margin-top: 0.5rem;
    }
</style>
