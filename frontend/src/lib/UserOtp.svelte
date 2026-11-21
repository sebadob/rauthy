<script lang="ts">
    import { formatDateFromTs } from '#utils/helpers.ts';
    import MfaCredential from '#lib/MfaCredential.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { OtpResponse } from '#api/types/otp.ts';
    import { I18nEn } from '../i18n/common/en';

    let {
        otp,
        showInactive,
        onDelete,
    }: {
        otp: OtpResponse;
        showInactive: boolean;
        onDelete: (id: string, name: string) => void;
    } = $props();

    let t = useI18n();
    let name = $derived(otp.name || (otp.kind === 'email' ? t.mfa.otp.titleEmail : otp.kind));
</script>

{#if otp.is_active || showInactive}
    <MfaCredential
        {name}
        metadata={[
            {
                label: t.mfa.lastUsed,
                value: otp.last_used ? formatDateFromTs(otp.last_used) : t.common.never,
            },
        ]}
        onDelete={() => onDelete(otp.id, name)}
    >
        {#snippet badges()}
            <span class="status" class:active={otp.is_active}>
                {otp.is_active
                    ? (t.mfa.otp.active ?? I18nEn.mfa.otp.active)
                    : (t.mfa.otp.inactive ?? I18nEn.mfa.otp.inactive)}
            </span>
        {/snippet}
    </MfaCredential>
{/if}

<style>
    .status {
        padding: 0 0.375rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        color: var(--ui-muted);
        font-size: 0.75rem;
    }

    .active {
        color: var(--ui-text);
        background: var(--ui-tonal);
    }
</style>
