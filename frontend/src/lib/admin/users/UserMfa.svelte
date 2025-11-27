<script lang="ts">
    import type { UserResponse } from '$api/types/user.ts';
    import { fetchDelete, fetchGet } from '$api/fetch';
    import type { PasskeyResponse } from '$api/types/webauthn.ts';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import UserPasskey from '$lib5/UserPasskey.svelte';

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

    $effect(() => {
        fetchPasskeys();
    });

    async function fetchPasskeys() {
        let res = await fetchGet<PasskeyResponse[]>(`/auth/v1/users/${user.id}/webauthn`);
        if (res.body) {
            passkeys = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function onDelete(name: string) {
        let isLastKey = passkeys.length === 1;

        let res = await fetchDelete(`/auth/v1/users/${user.id}/webauthn/delete/${name}`, {});
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
</script>

{#if passkeys.length === 0}
    <p>{ta.users.noMfaKeys}</p>
{:else}
    <p>{ta.users.mfaDelete1}</p>
    <p>{@html ta.users.mfaDelete2}</p>

    <div class="keysContainer">
        {#each passkeys as passkey (passkey.name)}
            <UserPasskey
                {passkey}
                showDelete
                {onDelete}
            />
        {/each}
    </div>
{/if}

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<style>
    .keysContainer {
        margin-top: 1rem;
        max-height: 30rem;
        overflow-y: auto;
    }
</style>
