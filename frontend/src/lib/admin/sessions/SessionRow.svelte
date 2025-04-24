<script lang="ts">
    import {formatDateFromTs} from "$utils/helpers.ts";
    import type {SessionResponse} from "$api/types/session.ts";
    import CheckIcon from "$lib5/CheckIcon.svelte";
    import Expandable from "$lib5/Expandable.svelte";
    import Tooltip from "$lib5/Tooltip.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import Button from "$lib5/button/Button.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import IconTrash from "$icons/IconTrash.svelte";
    import {fetchDelete} from "$api/fetch.ts";

    let {
        session,
        now,
        onDeleted,
    }: {
        session: SessionResponse,
        now: number,
        onDeleted: (sid: string) => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

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

    async function deleteSession() {
        let res = await fetchDelete(`/auth/v1/sessions/id/${session.id}`);
        if (res.error) {
            console.error(res.error);
        } else {
            onDeleted(session.id);
        }
    }
</script>

<div class="container" class:expired={session.exp < now}>
    <Expandable>
        {#snippet summary()}
            <div class="header">
                <Tooltip text="User ID">
                    <div class="uid font-mono" style:margin-left={session.user_id ? '' : '.35rem'}>
                        {session.user_id}
                    </div>
                </Tooltip>

                <div class="headerRight">
                    <Tooltip text={ta.options.lastSeen}>
                        <div class="date">
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

                    <Button invisible onclick={deleteSession}>
                        <Tooltip text={t.common.delete}>
                            <div class="trash">
                                <IconTrash width="1.2rem"/>
                            </div>
                        </Tooltip>
                    </Button>
                </div>
            </div>
        {/snippet}

        {#snippet details()}
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
                <CheckIcon checked={session.is_mfa}/>
            </LabeledValue>

            <Button level={-1} onclick={deleteSession}>
                {t.common.delete}
            </Button>
        {/snippet}
    </Expandable>
</div>

<style>
    .container {
        max-width: 48.5rem;
    }

    .header {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0 .5rem;
    }

    .headerRight {
        display: flex;
        align-items: center;
        gap: .5rem;
    }

    .uid {
        min-width: 14rem;
    }

    .ip {
        color: hsl(var(--accent));
        cursor: copy;
    }

    .date {
        color: hsla(var(--text) / .8);
    }

    .expired {
        background: hsl(var(--bg-high));
    }

    .trash {
        margin-bottom: -.3rem;
    }
</style>
