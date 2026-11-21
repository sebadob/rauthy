<script lang="ts">
    import type { Snippet } from 'svelte';
    import AdminShell from '#lib/admin/AdminShell.svelte';
    import Button from '#lib/button/Button.svelte';
    import { useSession } from '#state/session.svelte.ts';
    import { fetchGet } from '#api/fetch.ts';
    import { initI18nAdmin, useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        children,
    }: {
        children: Snippet;
    } = $props();

    initI18nAdmin();

    let ta = useI18nAdmin();
    let session = useSession('admin');
    let isAdmin = $state(false);
    let needsAdminRole = $state(false);
    let mfaReqErr = $state(false);

    $effect(() => {
        let s = session.get();
        if (s) {
            // full Rauthy admins and delegated group admins both reach the admin UI;
            // the backend still scopes every action, and the nav below hides what group
            // admins cannot do
            if (session.isAnyAdmin()) {
                isAdmin = true;
                // async check for admin access speeds up FCP and is still fast enough for a good UX
                checkAdminAccess();
            } else {
                needsAdminRole = true;
            }
        }
    });

    async function checkAdminAccess() {
        let res = await fetchGet('/auth/v1/auth_check_admin');
        if (res.status === 406) {
            mfaReqErr = true;
        }
    }
</script>

<svelte:head>
    <title>Rauthy Admin</title>
</svelte:head>

{#if mfaReqErr}
    <div class="noAdmin">
        <div>
            <div class="text">
                {@html ta.error.noAdmin}
            </div>
            <Button onclick={() => (window.location.href = '/auth/v1/account')}
                >{ta.common.account}</Button
            >
        </div>
    </div>
{:else if needsAdminRole}
    <div class="noAdmin">
        <div>
            <div class="text">
                {@html ta.error.needsAdminRole}
            </div>
            <Button onclick={() => (window.location.href = '/auth/v1/')}>{ta.common.back}</Button>
        </div>
    </div>
{:else if isAdmin}
    <AdminShell {children} />
{/if}

<style>
    .noAdmin {
        width: 100dvw;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .text {
        margin-bottom: 1rem;
    }
</style>
