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
        UserConsentsDeleteResponse,
    } from '#api/types/user.ts';
    import { formatDateFromTs, redirectToLogin } from '#utils/helpers.ts';

    let {
        userId,
        description,
        empty,
        revokeDesc,
        revokeAllDesc,
        sessionsWarning,
        loginState,
    }: {
        userId: string;
        description: string;
        empty: string;
        revokeDesc: string;
        revokeAllDesc: string;
        // a revoke deletes all sessions of the user, the confirm dialog says so
        sessionsWarning: string;
        // set when the revoke ends the session of the viewer itself: after a revoke, the next
        // request would fail with a 401, so the viewer is sent to this login instead
        loginState?: 'account' | 'admin';
    } = $props();

    let t = useI18n();

    let err = $state('');
    let isRevoking = $state(false);
    let consents = $state<UserConsentResponse[]>();
    // `'all'` for the revoke of all consents
    let target = $state<UserConsentResponse | 'all'>();
    let showRevoke = $state(false);
    let closeRevoke = $state<() => void>();

    onMount(() => {
        fetchConsents();
    });

    async function fetchConsents() {
        err = '';
        let res = await fetchGet<UserConsentResponse[]>(`/auth/v1/users/${userId}/consents`);
        if (res.body) {
            consents = res.body;
        } else {
            err = res.error?.message || 'Error fetching consents';
        }
    }

    function confirmRevoke(consent: UserConsentResponse | 'all') {
        err = '';
        target = consent;
        showRevoke = true;
    }

    async function onRevoke(consent: UserConsentResponse | 'all') {
        if (isRevoking) {
            return;
        }
        isRevoking = true;
        err = '';

        let redirecting = false;
        try {
            let error;
            if (consent === 'all') {
                let res = await fetchDelete<UserConsentsDeleteResponse>(
                    `/auth/v1/users/${userId}/consents/all`,
                );
                error = res.error;
            } else {
                let params = new URLSearchParams({ client_id: consent.client_id });
                if (consent.resource) {
                    params.set('resource', consent.resource);
                }
                let res = await fetchDelete<UserConsentDeleteResponse>(
                    `/auth/v1/users/${userId}/consents?${params}`,
                );
                error = res.error;
            }

            if (error) {
                err = error.message;
            } else if (loginState) {
                // the revoke deleted the viewer's own sessions
                redirecting = true;
                redirectToLogin(loginState);
            } else {
                closeRevoke?.();
                await fetchConsents();
            }
        } finally {
            // keep the buttons disabled until the browser leaves the page
            isRevoking = redirecting;
        }
    }

    function isoUtc(ts: number) {
        return new Date(ts * 1000).toISOString();
    }
</script>

{#snippet timestamp(ts: number)}
    <time datetime={isoUtc(ts)} title={`${isoUtc(ts)} (UTC)`}>{formatDateFromTs(ts)}</time>
{/snippet}

<div class="ui-section-stack">
    <div class="head">
        <p class="ui-muted-copy">{description}</p>
        {#if consents && consents.length > 0}
            <Button level={-3} onclick={() => confirmRevoke('all')}>
                {t.account.privacy.revokeAll}
            </Button>
        {/if}
    </div>

    {#if consents}
        {#each consents as consent (JSON.stringify([consent.client_id, consent.resource ?? null]))}
            <Section title={consent.client_name || consent.client_id}>
                {#snippet actions()}
                    <Button level={-3} onclick={() => confirmRevoke(consent)}>
                        {t.account.privacy.revoke}
                    </Button>
                {/snippet}
                <div class="ui-field-grid">
                    <LabeledValue label={t.account.privacy.clientId} mono
                        >{consent.client_id}</LabeledValue
                    >
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
                        >{@render timestamp(consent.created)}</LabeledValue
                    >
                    <LabeledValue label={t.account.privacy.updated}
                        >{@render timestamp(consent.updated)}</LabeledValue
                    >
                    {#if consent.user_agent}
                        <LabeledValue label={t.account.privacy.userAgent}
                            >{consent.user_agent}</LabeledValue
                        >
                    {/if}
                </div>
            </Section>
        {:else}
            {#if !err}<Section><p class="ui-muted-copy">{empty}</p></Section>{/if}
        {/each}
    {:else if !err}
        <Loading />
    {/if}

    {#if err && !showRevoke}<p class="err" role="alert">{err}</p>{/if}
</div>

<Modal
    bind:showModal={showRevoke}
    bind:closeModal={closeRevoke}
    ariaLabel={target === 'all' ? t.account.privacy.revokeAll : t.account.privacy.revoke}
    strict
>
    {#if target}
        {#if target === 'all'}
            <h3>{t.account.privacy.revokeAll}</h3>
            <p class="ui-muted-copy description">{revokeAllDesc}</p>
        {:else}
            <h3>{t.account.privacy.revoke}</h3>
            <p><strong>{target.client_name || target.client_id}</strong></p>
            <p class="ui-muted-copy description">{revokeDesc}</p>
        {/if}
        <p class="warning">{sessionsWarning}</p>
        {#if err}<p class="err" role="alert">{err}</p>{/if}
        <div class="actions">
            <Button level={2} onclick={() => closeRevoke?.()} isDisabled={isRevoking}
                >{t.common.cancel}</Button
            >
            <Button level={-1} isLoading={isRevoking} onclick={() => target && onRevoke(target)}>
                {target === 'all' ? t.account.privacy.revokeAll : t.account.privacy.revoke}
            </Button>
        </div>
    {/if}
</Modal>

<style>
    .head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .head p {
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
        margin-block: 0.5rem 0;
    }

    .warning {
        margin-block: 0.5rem 1rem;
        font-weight: 600;
    }

    .actions {
        justify-content: flex-end;
    }
</style>
