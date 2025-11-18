<script lang="ts">
    import NavSub from "$lib5/nav/NavSub.svelte";
    import NavLinkSub from "$lib5/nav/NavLinkSub.svelte";
    import type {Snippet} from "svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useTrigger} from "$state/callback.svelte";

    let {children}: { children: Snippet } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let refA: undefined | HTMLAnchorElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refA?.focus());

</script>

<svelte:head>
    <title>Rauthy Config</title>
</svelte:head>

<NavSub width="11rem" paddingTop="9.5rem">
    <NavLinkSub bind:ref={refA} href="/auth/v1/admin/config/policy">
        {t.passwordPolicy.passwordPolicy}
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/jwks">
        JWKS
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/argon2">
        Password Hashing
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/encryption">
        {ta.docs.encryption}
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/backups">
        Backups
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/tos">
        {ta.tos.tos}
    </NavLinkSub>
</NavSub>

<ContentAdmin>
    {@render children()}
</ContentAdmin>
