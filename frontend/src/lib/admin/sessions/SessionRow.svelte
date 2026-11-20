<script lang="ts">
    import { formatDateFromTs } from '#utils/helpers.ts';
    import type { SessionResponse } from '#api/types/session.ts';
    import CheckIcon from '#lib/CheckIcon.svelte';
    import Expandable from '#lib/Expandable.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Button from '#lib/button/Button.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import { useSession } from '#state/session.svelte.ts';

    let {
        session,
        now,
        onDeleted,
    }: {
        session: SessionResponse;
        now: number;
        onDeleted: (sid: string) => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    // deleting a single session is a full-admin action
    let admin = useSession('admin');
    let isAdmin = $derived(admin.isAdmin());

    let copyText = $state(t.common.copyToClip);

    function copyToClip() {
        if (session.remote_ip) {
            navigator.clipboard.writeText(session.remote_ip);
            copyText = ta.common.copiedToClip;
            setTimeout(() => {
                copyText = t.common.copyToClip;
            }, 3000);
        }
    }
</script>

<div class="container admin-session-card" class:expired={session.exp < now}>
    <Expandable>
        {#snippet summary()}
            <div class="header admin-session-summary">
                <div class="admin-session-identity">
                    <span class="admin-session-state">{session.state}</span>
                    <Tooltip text="User ID">
                        <div
                            class="uid font-mono"
                            style:margin-left={session.user_id ? '' : '.35rem'}
                        >
                            {session.user_id}
                        </div>
                    </Tooltip>
                </div>
                <div class="headerRight" class:withDelete={isAdmin}>
                    <Tooltip text={ta.options.lastSeen}>
                        <div class="date">
                            <span class="admin-field-caption">{ta.options.lastSeen}</span>
                            {formatDateFromTs(session.last_seen)}
                        </div>
                    </Tooltip>

                    <Button invisible onclick={copyToClip}>
                        <Tooltip text={`IP - ${copyText}`}>
                            <div class="ip font-mono">
                                {session.remote_ip}
                            </div>
                        </Tooltip>
                    </Button>

                    {#if isAdmin}
                        <EntityDelete
                            url={`/auth/v1/sessions/id/${session.id}`}
                            name={session.id}
                            iconOnly
                            onDelete={() => onDeleted(session.id)}
                        />
                    {/if}
                </div>
            </div>
        {/snippet}

        {#snippet details()}
            <div class="admin-session-details">
                <LabeledValue label="Session ID">
                    {session.id}
                </LabeledValue>

                <LabeledValue label="User ID">
                    {session.user_id}
                </LabeledValue>

                <LabeledValue label={ta.options.expires}>
                    {formatDateFromTs(session.exp)}
                </LabeledValue>

                <LabeledValue label={ta.options.lastSeen}>
                    {formatDateFromTs(session.last_seen)}
                </LabeledValue>

                <LabeledValue label={ta.options.state}>
                    {session.state}
                </LabeledValue>

                <LabeledValue label="IP">
                    {session.remote_ip}
                </LabeledValue>

                <LabeledValue label="MFA">
                    <CheckIcon checked={session.is_mfa} />
                </LabeledValue>
            </div>
        {/snippet}
    </Expandable>
</div>

<style>
    .expired {
        background: var(--ui-container);
    }
    .admin-session-summary {
        display: grid;
        align-items: center;
        grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
        gap: 1rem;
    }
    .admin-session-identity {
        display: flex;
        flex-direction: column;
        gap: 0.375rem;
        min-width: 0;
    }
    .admin-session-state {
        width: fit-content;
        padding: 2px 8px;
        border-radius: var(--border-radius);
        background: var(--ui-tonal);
        color: var(--ui-primary);
        font-size: 0.75rem;
        font-weight: 600;
    }
    .uid,
    .ip {
        overflow-wrap: anywhere;
        font-size: 0.75rem;
        line-height: 1.25rem;
    }
    .headerRight {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        align-items: center;
        gap: 0.625rem;
    }
    .headerRight.withDelete {
        grid-template-columns: repeat(2, minmax(0, 1fr)) auto;
    }
    .headerRight > :global(.admin-control) {
        min-width: 0;
    }
    .admin-field-caption {
        display: block;
        font-size: 0.75rem;
        color: var(--ui-muted);
    }
    .date {
        font-size: 0.8125rem;
        line-height: 1.25rem;
    }
    .admin-session-details {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 0.75rem 1.5rem;
    }
    @media (max-width: 800px) {
        .admin-session-summary,
        .admin-session-details {
            grid-template-columns: minmax(0, 1fr);
        }
    }
</style>
