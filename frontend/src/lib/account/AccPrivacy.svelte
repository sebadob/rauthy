<script lang="ts">
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
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
    let consents: UserConsentResponse[] = $state([]);

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
        err = '';

        let params = new URLSearchParams({
            client_id: consent.client_id,
            resource: consent.resource,
        });
        let res = await fetchDelete<UserConsentDeleteResponse>(
            `/auth/v1/users/${user.id}/consents?${params}`,
        );
        if (res.error) {
            err = res.error.message;
        } else {
            await fetchConsents();
        }
    }
</script>

<div class="container">
    <h3>{t.account.privacy.connectedApps}</h3>
    <p>{t.account.privacy.connectedAppsDesc}</p>

    {#each consents as consent (`${consent.client_id}/${consent.resource}`)}
        <div class="consent">
            <div class="row">
                <b>{consent.client_name || consent.client_id}</b>
            </div>
            {#if consent.client_name}
                <div class="row">
                    <span class="label">{t.account.privacy.clientId}</span>
                    <span class="font-mono">{consent.client_id}</span>
                </div>
            {/if}
            {#if consent.resource}
                <div class="row">
                    <span class="label">{t.account.privacy.resource}</span>
                    <span class="font-mono">{consent.resource}</span>
                </div>
            {/if}
            <div class="row">
                <span class="label">{t.account.privacy.scopes}</span>
                <span class="font-mono">{consent.scopes.join(' ')}</span>
            </div>
            <div class="row">
                <span class="label">{t.account.privacy.granted}</span>
                <span class="font-mono">{formatDateFromTs(consent.created)}</span>
            </div>
            {#if consent.updated !== consent.created}
                <div class="row">
                    <span class="label">{t.account.privacy.updated}</span>
                    <span class="font-mono">{formatDateFromTs(consent.updated)}</span>
                </div>
            {/if}
            <div class="row">
                <div></div>
                <div class="revokeBtn" title={t.account.privacy.revokeDesc}>
                    <Button level={-3} onclick={() => onRevoke(consent)}>
                        {t.account.privacy.revoke}
                    </Button>
                </div>
            </div>
        </div>
    {:else}
        <p class="label">{t.account.privacy.noConnectedApps}</p>
    {/each}

    {#if err}
        <div class="err" transition:fade>
            {err}
        </div>
    {/if}
</div>

<style>
    .consent {
        margin: 0.5rem 0;
        padding-bottom: 0.5rem;
        border-bottom: 1px solid hsl(var(--bg-high));
        overflow: clip;
    }

    .container {
        display: flex;
        flex-direction: column;
    }

    .err {
        margin-top: 0.5rem;
        color: hsl(var(--error));
    }

    .label {
        color: hsla(var(--text) / 0.7);
    }

    .revokeBtn {
        margin-right: -0.35rem;
    }

    .row {
        width: min(95dvw, 30rem);
        display: flex;
        gap: 0.5rem;
        justify-content: space-between;
        align-items: center;
    }

    .row .font-mono {
        text-align: right;
        overflow-wrap: anywhere;
    }
</style>
