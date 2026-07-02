<script lang="ts">
    import { formatDateFromTs } from '$utils/helpers';
    import Button from '$lib5/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import type { OtpResponse } from '$api/types/otp';
    import IconCheckBadge from '$icons/IconCheckBadge.svelte';
    import IconStop from '$icons/IconStop.svelte';

    let {
        otp,
        showInactive,
        onDelete,
    }: {
        otp: OtpResponse;
        showInactive: boolean;
        onDelete: (name: string) => void;
    } = $props();

    let t = useI18n();
</script>

{#if otp.is_active || showInactive}
    <div class="keyContainer">
        <div class="row">
            <span class="label">
                {#if otp.kind == 'email'}
                    {t.mfa.otp.titleEmail}
                {:else}
                    <!-- todo: implement other kind of otp -->
                    Not implemented
                {/if}
            </span>

            <div class="name">
                <b>{otp.name}</b>
                {#if otp.is_active}
                    <div style:margin-bottom="-.25rem">
                        <IconCheckBadge width="1rem" color="hsl(var(--accent))" />
                    </div>
                {:else}
                    <div style:margin-bottom="-.25rem">
                        <IconStop width="1rem" color="hsl(var(--accent))" />
                    </div>
                {/if}
            </div>
        </div>
        <div class="row">
            <span class="label">
                {t.mfa.lastUsed}
            </span>
            <span class="font-mono">{formatDateFromTs(otp.last_used)}</span>
        </div>

        <div class="row">
            <div></div>
            <div class="deleteBtn">
                <Button level={-3} onclick={() => onDelete(otp.id.toString())}>
                    {t.common.delete}
                </Button>
            </div>
        </div>
    </div>
{/if}

<style>
    .deleteBtn {
        margin-right: -0.35rem;
    }

    .keyContainer {
        margin: 0.33rem 0;
        overflow: clip;
    }

    .label {
        color: hsla(var(--text) / 0.7);
    }

    .name {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
    }

    .row {
        width: min(95dvw, 21rem);
        display: flex;
        gap: 0.5rem;
        justify-content: space-between;
        align-items: center;
    }
</style>
