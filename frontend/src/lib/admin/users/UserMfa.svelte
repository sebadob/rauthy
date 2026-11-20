<script lang="ts">
    import Modal from '#lib/Modal.svelte';
    import MfaMethod from '#lib/MfaMethod.svelte';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { UserResponse } from '#api/types/user.ts';
    import { fetchDelete, fetchGet } from '#api/fetch.ts';
    import type { PasskeyResponse } from '#api/types/webauthn.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import UserPasskey from '#lib/UserPasskey.svelte';
    import type { OtpResponse } from '#api/types/otp.ts';
    import Template from '#lib/Template.svelte';
    import { TPL_IS_OTP_ENABLED } from '#utils/constants.ts';
    import { deleteOtp } from '#mfa/otp/mod.ts';
    import UserOtp from '#lib/UserOtp.svelte';

    const t = useI18n();
    let showConfirm = $state(false);
    let closeConfirm: (() => void) | undefined = $state();
    let target: { kind: 'passkey' | 'otp'; id: string; name: string } | undefined = $state();
    let deleting = $state(false);

    function ask(kind: 'passkey' | 'otp', id: string, name: string) {
        err = '';
        target = { kind, id, name };
        showConfirm = true;
    }

    async function remove() {
        if (!target) return;
        deleting = true;
        if (target.kind === 'passkey') await onDelete(target.id);
        else await onDeleteOtp(target.id);
        deleting = false;
        if (!err) closeConfirm?.();
    }

    let {
        user,
        onSave,
    }: {
        user: UserResponse;
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();

    let err = $state('');
    let passkeys: PasskeyResponse[] = $state([]);
    let loadingPasskeys = $state(true);
    let passkeyError = $state<string>();

    let otps: OtpResponse[] = $state([]);
    let loadingOtps = $state(true);
    let otpError = $state<string>();
    let isOtpEnabled = $state(false);

    $effect(() => {
        fetchPasskeys();
    });

    $effect(() => {
        if (isOtpEnabled) {
            fetchOtps();
        }
    });

    async function fetchPasskeys() {
        let res = await fetchGet<PasskeyResponse[]>(`/auth/v1/users/${user.id}/webauthn`);
        loadingPasskeys = false;
        passkeyError = res.body ? undefined : res.error?.message || 'Error';
        if (res.body) passkeys = res.body;
    }

    async function onDelete(name: string) {
        let isLastKey = passkeys.length === 1;

        let res = await fetchDelete(
            `/auth/v1/users/${user.id}/webauthn/delete/${encodeURIComponent(name)}`,
            {},
        );
        if (res.status === 200) {
            await fetchPasskeys();

            // if this was the last key, we need to re-fetch the user to show the
            // correct "mfa enabled" status
            if (isLastKey) {
                onSave();
            }
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function fetchOtps() {
        let res = await fetchGet<OtpResponse[]>(`/auth/v1/users/${user.id}/otp`);
        loadingOtps = false;
        otpError = res.body ? undefined : res.error?.message || 'Error';
        if (res.body) otps = res.body;
    }

    async function onDeleteOtp(id: string) {
        let res = await deleteOtp(user.id, id, undefined);
        if (res.error) {
            err = res.error || 'Error';
        } else {
            await fetchOtps();
        }
    }
</script>

<Template id={TPL_IS_OTP_ENABLED} bind:value={isOtpEnabled} />

<div class="methods">
    <MfaMethod
        title={ta.users.mfa.webauthn.title}
        loading={loadingPasskeys}
        error={passkeyError}
        empty={passkeys.length === 0 ? ta.users.mfa.webauthn.noMfaKeys : undefined}
    >
        {#each passkeys as passkey (passkey.name)}
            <UserPasskey {passkey} showDelete onDelete={name => ask('passkey', name, name)} />
        {/each}
    </MfaMethod>

    {#if isOtpEnabled}
        <MfaMethod
            title={ta.users.mfa.otp.title}
            loading={loadingOtps}
            error={otpError}
            empty={otps.length === 0 ? ta.users.mfa.otp.noMfaOtps : undefined}
        >
            {#each otps as otp (otp.id)}
                <UserOtp
                    {otp}
                    showInactive
                    onDelete={id => ask('otp', id, otp.name.toString() || id)}
                />
            {/each}
        </MfaMethod>
    {/if}
</div>

<Modal
    bind:showModal={showConfirm}
    bind:closeModal={closeConfirm}
    ariaLabel={t.common.delete}
    strict
>
    <h3>{t.common.delete}</h3>
    <p><strong>{target?.name}</strong></p>
    {#if target}
        <p class="warning">
            {@html (target.kind === 'passkey' ? ta.users.mfa.webauthn : ta.users.mfa.otp)
                .mfaDelete2}
        </p>
    {/if}
    {#if err}<p class="err" role="alert">{err}</p>{/if}
    <div class="flex gap-10">
        <Button level={2} onclick={() => closeConfirm?.()} isDisabled={deleting}
            >{t.common.cancel}</Button
        >
        <Button level={-1} onclick={remove} isLoading={deleting}>{t.common.delete}</Button>
    </div>
</Modal>

<style>
    .methods {
        display: grid;
        gap: var(--ui-space);
    }

    .warning {
        max-width: 38ch;
        margin-block: 0.5rem 1rem;
    }
</style>
