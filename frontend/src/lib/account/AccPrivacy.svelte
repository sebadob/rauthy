<script lang="ts">
    import { onMount } from 'svelte';
    import Section from '#lib/Section.svelte';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import Loading from '#lib/Loading.svelte';
    import Modal from '#lib/Modal.svelte';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.js';
    import { fetchDelete, fetchGet } from '#api/fetch.ts';
    import type {
        UserConsentDeleteResponse,
        UserConsentResponse,
        UserResponse,
    } from '#api/types/user.ts';
    import { formatDateFromTs } from '#utils/helpers.ts';

    let {
        user,
    }: {
        user: UserResponse;
    } = $props();

    let t = useI18n();

    let err = $state('');
    let isRevoking = $state(false);
    let consents = $state<UserConsentResponse[]>();
    let target = $state<UserConsentResponse>();
    let showRevoke = $state(false);
    let closeRevoke = $state<() => void>();

    onMount(() => {
        fetchConsents();
    });

    async function fetchConsents() {
        err = '';
        let res = await fetchGet<UserConsentResponse[]>(`/auth/v1/users/${user.id}/consents`);
        if (res.body) {
            consents = res.body;
        } else {
            err = res.error?.message || 'Error fetching consents';
        }
    }

    async function onRevoke(consent: UserConsentResponse) {
        if (isRevoking) {
            return;
        }
        isRevoking = true;
        err = '';

        try {
            let params = new URLSearchParams({ client_id: consent.client_id });
            if (consent.resource) {
                params.set('resource', consent.resource);
            }
            let res = await fetchDelete<UserConsentDeleteResponse>(
                `/auth/v1/users/${user.id}/consents?${params}`,
            );
            if (res.error) {
                err = res.error.message;
            } else {
                closeRevoke?.();
                await fetchConsents();
            }
        } finally {
            isRevoking = false;
        }
    }
</script>

<div class="ui-section-stack">
    <h2 class="ui-page-title">{t.account.privacy.connectedApps}</h2>
    <p class="ui-muted-copy">{t.account.privacy.connectedAppsDesc}</p>

    {#if consents}
        {#each consents as consent (JSON.stringify([consent.client_id, consent.resource ?? null]))}
            <Section title={consent.client_name || consent.client_id}>
                {#snippet actions()}
                    <Button
                        level={-3}
                        onclick={() => {
                            err = '';
                            target = consent;
                            showRevoke = true;
                        }}
                    >
                        {t.account.privacy.revoke}
                    </Button>
                {/snippet}
                <div class="ui-field-grid">
                    {#if consent.client_name}
                        <LabeledValue label={t.account.privacy.clientId} mono
                            >{consent.client_id}</LabeledValue
                        >
                    {/if}
                    {#if consent.resource}
                        <LabeledValue label={t.account.privacy.resource} mono
                            >{consent.resource}</LabeledValue
                        >
                    {/if}
                    <LabeledValue label={t.account.privacy.scopes}>
                        <div class="scopes">
                            {#each consent.scopes as scope (scope)}<code>{scope}</code>{/each}
                        </div>
                    </LabeledValue>
                    <LabeledValue label={t.account.privacy.granted}
                        >{formatDateFromTs(consent.created)}</LabeledValue
                    >
                    {#if consent.updated !== consent.created}
                        <LabeledValue label={t.account.privacy.updated}
                            >{formatDateFromTs(consent.updated)}</LabeledValue
                        >
                    {/if}
                    {#if consent.user_agent}
                        <LabeledValue label={t.account.privacy.userAgent}
                            >{consent.user_agent}</LabeledValue
                        >
                    {/if}
                </div>
            </Section>
        {:else}
            {#if !err}<Section
                    ><p class="ui-muted-copy">{t.account.privacy.noConnectedApps}</p></Section
                >{/if}
        {/each}
    {:else if !err}
        <Loading />
    {/if}

    {#if err && !showRevoke}<p class="err" role="alert">{err}</p>{/if}
</div>

<Modal
    bind:showModal={showRevoke}
    bind:closeModal={closeRevoke}
    ariaLabel={t.account.privacy.revoke}
    strict
>
    {#if target}
        <h3>{t.account.privacy.revoke}</h3>
        <p><strong>{target.client_name || target.client_id}</strong></p>
        <p class="ui-muted-copy description">{t.account.privacy.revokeDesc}</p>
        {#if err}<p class="err" role="alert">{err}</p>{/if}
        <div class="actions">
            <Button level={2} onclick={() => closeRevoke?.()} isDisabled={isRevoking}
                >{t.common.cancel}</Button
            >
            <Button level={-1} isLoading={isRevoking} onclick={() => target && onRevoke(target)}>
                {t.account.privacy.revoke}
            </Button>
        </div>
    {/if}
</Modal>

<style>
    h2 {
        margin: 0;
    }

    .scopes,
    .actions {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.375rem;
    }

    .scopes code {
        padding: 0 0.375rem;
        border-radius: var(--border-radius);
        background: var(--ui-hover);
    }

    .description {
        margin-block: 0.5rem 1rem;
    }

    .actions {
        justify-content: flex-end;
    }
</style>
