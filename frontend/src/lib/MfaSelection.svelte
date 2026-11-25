<script lang="ts">
    import { onMount } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';
    import IconKey from '#icons/IconKey.svelte';
    import IconEnvelope from '#icons/IconEnvelope.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { ActiveOtp } from '#api/types/authorize.ts';
    import type { OtpResponse } from '#api/types/otp.ts';

    let {
        activeOtps,
        isDisabled = false,
        onSelect,
        onError,
    }: {
        activeOtps?: ActiveOtp[] | OtpResponse[];
        isDisabled?: boolean;
        onSelect: (kind: 'webauthn' | 'otp') => void;
        onError: (error: string) => void;
    } = $props();

    let t = useI18n();

    let showModal = $state(false);
    // a selection unmounts this component, which must not count as a cancel
    let selected = false;

    onMount(() => {
        showModal = true;
    });

    function select(kind: 'webauthn' | 'otp') {
        selected = true;
        onSelect(kind);
    }

    function onModalClose() {
        if (!selected) {
            onError(t.mfa.requestCanceled);
        }
    }
</script>

<Modal bind:showModal onClose={onModalClose} ariaLabel={t.authorize.mfaSelectionDesc}>
    <div class="mfaSelection flex-col gap-05">
        <p>{t.authorize.mfaSelectionDesc}</p>

        <div class="btn flex-col">
            <Button
                ariaLabel={t.authorize.usePasskey}
                onclick={() => select('webauthn')}
                {isDisabled}
            >
                <div class="flex gap-05">
                    <IconKey width="1.2rem" />
                    {t.authorize.usePasskey}
                </div>
            </Button>
        </div>

        {#if activeOtps && activeOtps.length > 0}
            <div class="btn flex-col">
                <Button
                    level={2}
                    ariaLabel={t.authorize.useEmailCode}
                    onclick={() => select('otp')}
                    {isDisabled}
                >
                    <div class="flex gap-05">
                        <IconEnvelope width="1.2rem" />
                        {t.authorize.useEmailCode}
                    </div>
                </Button>
            </div>
        {/if}
    </div>
</Modal>

<style>
    .mfaSelection {
        max-width: 22rem;
    }

    .btn {
        margin: 0.8rem 0 0.5rem 0;
        display: flex;
    }

    .btn :global(button) {
        width: 100%;
    }
</style>
