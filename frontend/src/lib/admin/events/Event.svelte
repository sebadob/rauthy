<script lang="ts">
    import {eventColor, formatDateFromTs} from "$utils/helpers.ts";
    import {onMount} from "svelte";
    import type {EventResponse} from "$api/types/events.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import A from "$lib5/A.svelte";
    import Button from "$lib5/button/Button.svelte";
    import Tooltip from "$lib5/Tooltip.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";

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
        event: EventResponse,
        inline?: boolean,
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

{#snippet ip(v: undefined | string)}
    {#if v}
        <div>
            <Button invisible onclick={() => copyToClip(v)}>
                <Tooltip text={tooltip} yOffset={20}>
                    <div class="ip font-mono">
                        {v}
                    </div>
                </Tooltip>
            </Button>
        </div>
    {/if}
{/snippet}

<div
        role="contentinfo"
        class="event"
        class:highlight
        class:inline
        style:border-left={`2px solid ${eventColor(event.level)}`}
>
    <div class="ts">
        {formatDateFromTs(event.timestamp / 1000)}
    </div>

    {#if event.typ !== 'RauthyHealthy'
    && event.typ !== 'RauthyUnhealthy'
    && event.typ !== 'NewRauthyVersion'
    && event.typ !== 'Test'}
        {event.typ}
    {/if}

    {#if event.typ === 'InvalidLogins'}
        {`: ${event.data}`}
        {@render ip(event.ip)}

    {:else if event.typ === 'NewRauthyVersion'}
        <A href={event.text || ''} target="_blank">NewRauthyVersion</A>

    {:else if event.typ === 'IpBlacklisted'}
        {ta.common.until}
        {event.data && formatDateFromTs(event.data)}
        {@render ip(event.ip)}

    {:else}
        {event.text}
        {@render ip(event.ip)}
    {/if}
</div>

<style>
    .event {
        width: 100%;
        min-width: 5px;
        padding: .1rem .5rem .2rem .5rem;
        display: flex;
        flex-direction: column;
        line-height: 1rem;
        margin-bottom: .15rem;
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
        column-gap: .5rem;
        flex-wrap: wrap;
    }

    .ip {
        margin-top: -.05rem;
        font-size: .9rem;
        color: hsl(var(--accent));
        cursor: copy;
    }

    .ts {
        font-size: .9rem;
        color: hsla(var(--text) / .8);
        transition: color 150ms ease-in-out;
    }

    .highlight .ts {
        color: hsl(var(--bg) / .8);
    }
</style>
