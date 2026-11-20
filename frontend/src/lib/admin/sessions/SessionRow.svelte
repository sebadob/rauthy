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
                <div class="headerRight">
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
            {#if isAdmin}
                <EntityDelete
                    url={`/auth/v1/sessions/id/${session.id}`}
                    name={session.id}
                    onDelete={() => onDeleted(session.id)}
                />
            {/if}
        {/snippet}
    </Expandable>
</div>

<style>
    .container {
        max-width: 48.5rem;
        overflow: clip;
    }

    .header {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0 0.5rem;
    }

    .headerRight {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .uid {
        min-width: 14rem;
    }

    .ip {
        color: hsl(var(--accent));
        cursor: copy;
    }

    .date {
        color: hsla(var(--text) / 0.8);
    }

    .expired {
        background: hsl(var(--bg-high));
    }

    .admin-session-card {
        max-width: none;
        overflow: visible;
    }
    .admin-session-card .admin-session-summary {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
    }
    .admin-session-identity {
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }
    .admin-session-state {
        width: fit-content;
        padding: 2px 8px;
        background: var(--ui-tonal);
        color: var(--ui-primary);
        border-radius: var(--border-radius);
        font-size: 11px;
        line-height: 18px;
        font-weight: 600;
    }
    .admin-session-card :global(.uid) {
        min-width: 0;
        font-size: 12px;
        line-height: 20px;
        overflow-wrap: anywhere;
    }
    .admin-session-card :global(.headerRight) {
        gap: 16px;
        flex-wrap: wrap;
    }
    .admin-field-caption {
        display: block;
        font-size: 11px;
        line-height: 18px;
        color: var(--ui-muted);
    }
    .admin-session-card :global(.date) {
        font-size: 12px;
        line-height: 20px;
        white-space: normal;
    }
    .admin-session-details {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 12px 24px;
        margin-bottom: 12px;
    }
    @media (max-width: 800px) {
        .admin-session-card .admin-session-summary {
            align-items: flex-start;
            flex-direction: column;
            gap: 12px;
        }
        .admin-session-card :global(.headerRight) {
            gap: 8px;
        }
        .admin-session-details {
            grid-template-columns: 1fr;
        }
    }
    .admin-session-card .admin-session-summary {
        display: grid;
        grid-template-columns: minmax(140px, 1fr) minmax(0, 500px);
        gap: 20px;
    }
    .admin-session-card :global(.headerRight) {
        display: grid;
        grid-template-columns: 140px minmax(0, 1fr) 36px;
        width: 100%;
        gap: 12px;
    }
    .admin-session-card :global(.ip) {
        font-size: 11px;
        overflow-wrap: anywhere;
    }
    .admin-session-card :global(.headerRight) > :global(.admin-control) {
        min-width: 0;
    }
    @media (max-width: 800px) {
        .admin-session-card .admin-session-summary {
            display: flex;
            flex-direction: column;
            align-items: stretch;
        }
        .admin-session-card :global(.headerRight) {
            grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) 36px;
        }
    }
</style>
