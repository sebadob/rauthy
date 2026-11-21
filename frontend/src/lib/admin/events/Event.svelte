<script lang="ts">
    import { eventColor, formatDateFromTs } from '#utils/helpers.ts';
    import { onMount } from 'svelte';
    import type { EventResponse } from '#api/types/events.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import A from '#lib/A.svelte';
    import Button from '#lib/button/Button.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';

    // populated fields for different events:
    // - invalid_login: failed logins count as data, ip
    // - brute_force: ip
    // - ip_blacklisted: timestamp as data, ip
    // - ip_blacklist_removed: ip
    // - new_user: ip, email as text
    // - new_rauthy_admin: ip, email as text
    // - new_rauthy_version: version url as text
    // - jwks_rotated: empty
    // - rauthy_started: empty
    // - rauthy_unhealthy_cache: empty
    // - rauthy_unhealthy_db: empty
    // - secrets_migrated: ip
    // - test: ip
    // - user_email_change: Option<Ip>, text
    // - user_password_reset: Option<Ip>, text

    let {
        event,
        inline,
    }: {
        event: EventResponse;
        inline?: boolean;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let highlight = $state(false);
    let tooltip = $state(t.common.copyToClip);

    onMount(() => {
        let now = new Date().getTime();
        // this will only flash the event if it is not older than 30 seconds
        if (now - 1000 * 30 < event.timestamp) {
            setTimeout(() => {
                highlight = true;
            }, 100);
            setTimeout(() => {
                highlight = false;
            }, 2500);
        }
    });

    function copyToClip(v: string) {
        navigator.clipboard.writeText(v);
        tooltip = ta.common.copiedToClip;
        setTimeout(() => {
            tooltip = t.common.copyToClip;
        }, 3000);
    }
</script>

<article
    class="event"
    class:highlight
    class:inline
    style:--event-color={eventColor(event.level)}
    aria-label={`${ta.events.eventLevel}: ${event.level}`}
>
    <header class="admin-event-meta">
        <span class="admin-event-level">{event.level}</span>
        <time class="ts" datetime={new Date(event.timestamp).toISOString()}
            >{formatDateFromTs(event.timestamp / 1000)}</time
        >
    </header>
    <div class="admin-event-body">
        <strong>{event.typ.replace(/([a-z])([A-Z])/g, '$1 $2')}</strong>
        {#if event.typ === 'InvalidLogins'}<span>{event.data}</span>
        {:else if event.typ === 'NewRauthyVersion'}<A href={event.text || ''} target="_blank"
                >New Version</A
            >
        {:else if event.typ === 'IpBlacklisted'}<span
                >{ta.common.until} {event.data && formatDateFromTs(event.data)}</span
            >
        {:else if event.text}<p>{event.text}</p>{/if}
    </div>
    {#if event.ip}
        <div class="admin-event-footer">
            <Button ariaLabel="IP Address" invisible onclick={() => copyToClip(event.ip || '')}
                ><Tooltip text={tooltip} yOffset={20}
                    ><span class="ip font-mono">{event.ip}</span></Tooltip
                ></Button
            >
        </div>
    {/if}
</article>

<style>
    .event {
        width: 100%;
        min-width: 5px;
        padding: 0.1rem 0.5rem 0.2rem 0.5rem;
        display: flex;
        flex-direction: column;
        line-height: 1rem;
        margin-bottom: 0.15rem;
        word-break: break-word;
        background: transparent;
        color: hsl(var(--text));
        transition: background 150ms ease-in-out;
    }

    .highlight {
        background: hsl(var(--text));
        color: hsl(var(--bg));
    }

    .inline {
        flex-direction: row;
        align-items: center;
        column-gap: 0.5rem;
        flex-wrap: wrap;
    }

    .ip {
        margin-top: -0.05rem;
        font-size: 0.9rem;
        color: hsl(var(--accent));
        cursor: copy;
    }

    .ts {
        font-size: 0.9rem;
        color: hsla(var(--text) / 0.8);
        transition: color 150ms ease-in-out;
    }

    .highlight .ts {
        color: hsl(var(--bg) / 0.8);
    }

    .event {
        display: flex;
        flex-direction: column;
        width: 100%;
        min-width: 0;
        padding: 16px;
        margin: 0;
        border: 1px solid var(--ui-outline);
        border-left: 3px solid var(--event-color);
        border-radius: var(--border-radius);
        line-height: 22px;
        background: var(--ui-surface);
        color: var(--ui-text);
        overflow-wrap: anywhere;
    }
    .admin-event-meta {
        display: flex;
        align-items: center;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: 8px 16px;
        margin-bottom: 10px;
    }
    .admin-event-level {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        font-size: 11px;
        line-height: 18px;
        font-weight: 600;
        text-transform: capitalize;
    }
    .admin-event-level::before {
        content: '';
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: var(--event-color);
    }
    .event :global(.ts) {
        font-size: 12px;
        line-height: 20px;
        color: var(--ui-muted);
        white-space: normal;
        overflow: visible;
    }
    .admin-event-body {
        display: flex;
        flex-direction: column;
        gap: 4px;
        font-size: 13px;
    }
    .admin-event-body :global(strong) {
        font-weight: 600;
    }
    .admin-event-body :global(p) {
        margin: 0;
        color: var(--ui-muted);
    }
    .event :global(.ip) {
        margin: 0;
        font-size: 12px;
        color: var(--ui-muted);
    }
    .admin-event-footer {
        margin-top: 8px;
    }
    .admin-event-footer :global(.admin-control) {
        padding: 0;
        height: 28px;
        min-height: 28px;
        justify-content: flex-start;
    }

    .event {
        align-items: stretch;
        text-align: left;
    }

    @container archive (width >= 42rem) {
        .event.inline {
            display: grid;
            grid-template-columns: minmax(9rem, 1fr) minmax(0, 2fr) 9rem;
            align-items: start;
            column-gap: 1rem;
        }

        .event.inline .admin-event-meta {
            align-items: flex-start;
            justify-content: center;
            flex-direction: column;
            gap: 0.25rem;
            margin: 0;
        }

        .event.inline .admin-event-footer {
            min-width: 0;
            margin: 0;
        }
        .event.inline .admin-event-footer :global(.admin-control) {
            max-width: 100%;
            height: auto;
        }
    }
</style>
