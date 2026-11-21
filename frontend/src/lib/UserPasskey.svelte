<script lang="ts">
    import type { PasskeyResponse } from '#api/types/webauthn.ts';
    import { formatDateFromTs } from '#utils/helpers.ts';
    import IconFingerprint from '#icons/IconFingerprint.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import MfaCredential from '#lib/MfaCredential.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import IconKey from '#icons/IconKey.svelte';
    import IconCheckBadge from '#icons/IconCheckBadge.svelte';

    let {
        passkey,
        showDelete,
        onDelete,
    }: {
        passkey: PasskeyResponse;
        showDelete: boolean;
        onDelete: (name: string) => void;
    } = $props();

    let t = useI18n();
</script>

<MfaCredential
    name={passkey.name}
    metadata={[
        { label: t.mfa.registered, value: formatDateFromTs(passkey.registered) },
        {
            label: t.mfa.lastUsed,
            value: passkey.last_used ? formatDateFromTs(passkey.last_used) : t.common.never,
        },
    ]}
    onDelete={showDelete ? () => onDelete(passkey.name) : undefined}
>
    {#snippet badges()}
        {#if passkey.user_verified}
            <Tooltip text={t.account.userVerifiedTooltip}>
                <span class="badge"><IconFingerprint width="1rem" /></span>
            </Tooltip>
        {/if}
        {#if passkey.resident_key}
            <Tooltip text={t.account.residentKeyTooltip}>
                <span class="badge"><IconKey width="1rem" /></span>
            </Tooltip>
        {/if}
        {#if passkey.aaguid}
            <Tooltip text={`${t.account.aaguidTooltip}: ${passkey.description}`}>
                <span class="badge"><IconCheckBadge width="1rem" /></span>
            </Tooltip>
        {/if}
    {/snippet}
</MfaCredential>

<style>
    .badge {
        display: flex;
        color: var(--ui-muted);
    }
</style>
