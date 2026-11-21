<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import NavSub from '#lib/nav/NavSub.svelte';
    import NavLinkSub from '#lib/nav/NavLinkSub.svelte';
    import type { Snippet } from 'svelte';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import AdminListHeading from '#lib/admin/AdminListHeading.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';

    let { children }: { children: Snippet } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    function showList() {
        const url = new URL(page.url.href);
        url.searchParams.set('list', '');
        void goto(url);
    }
</script>

<svelte:head>
    <title>Rauthy Config</title>
</svelte:head>

<NavSub detail={!page.url.searchParams.has('list')} onBack={showList}>
    <AdminListHeading title={ta.nav.config} />
    <NavLinkSub href="/auth/v1/admin/config/policy">
        {t.passwordPolicy.passwordPolicy}
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/jwks">JWKS</NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/argon2">Password Hashing</NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/encryption">
        {ta.docs.encryption}
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/backups">{ta.backup.title ?? 'Backups'}</NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/tos">
        {ta.tos.tos}
    </NavLinkSub>
</NavSub>

<ContentAdmin>
    {@render children()}
</ContentAdmin>
