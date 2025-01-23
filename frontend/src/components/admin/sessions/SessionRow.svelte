<script>
    import {formatDateFromTs} from "../../../utils/helpers";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [session]
     * @property {number} [now]
     */

    /** @type {Props} */
    let {session = $bindable({}), now = $bindable(0)} = $props();
</script>

<div class="container" class:expired={session.exp < now}>
    <ExpandContainer>
        {#snippet header()}
            <div class="header">
                <Tooltip text="User ID">
                    <div class="uid">
                        {session.user_id}
                    </div>
                </Tooltip>

                <div class="header">
                    <Tooltip text="Peer IP">
                        <div class="ip">
                            {session.remote_ip}
                        </div>
                    </Tooltip>

                    <Tooltip text="Last Seen">
                        <div class="date">
                            {formatDateFromTs(session.last_seen)}
                        </div>
                    </Tooltip>
                </div>
            </div>
        {/snippet}

        {#snippet body()}
            <div class="details">
                <div class="flex">
                    <div class="label">Session ID:</div>
                    {session.id}
                </div>

                <div class="flex">
                    <div class="label">User ID:</div>
                    {session.user_id}
                </div>

                <div class="flex">
                    <div class="label">Expires:</div>
                    {formatDateFromTs(session.exp)}
                </div>

                <div class="flex">
                    <div class="label">Last Seen:</div>
                    {formatDateFromTs(session.last_seen)}
                </div>

                <div class="flex">
                    <div class="label">State:</div>
                    {session.state}
                </div>

                <div class="flex">
                    <div class="label">IP:</div>
                    {session.remote_ip}
                </div>

                <div class="flex">
                    <div class="label">MFA:</div>
                    <CheckIcon check={session.is_mfa}/>
                </div>
            </div>
        {/snippet}
    </ExpandContainer>
</div>

<style>
    .container {
        max-width: 48.5rem;
    }

    .header {
        display: flex;
        flex-wrap: wrap;
    }

    .uid {
        min-width: 14rem;
    }

    .ip {
        width: 8rem;
        text-align: right;
    }

    .date {
        width: 10rem;
        text-align: right;
    }

    .details {
        padding: 1rem;
    }

    .expired {
        background: hsl(var(--bg-high));
    }

    .flex {
        display: flex;
        align-items: center;
    }

    .label {
        width: 6rem;
        margin-right: .25rem;
        font-weight: bold;
        font-size: .9rem;
    }
</style>