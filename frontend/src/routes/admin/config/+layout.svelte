<script lang="ts">
    import NavSub from "$lib5/nav/NavSub.svelte";
    import NavLinkSub from "$lib5/nav/NavLinkSub.svelte";
    import type {Snippet} from "svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {useI18n} from "$state/i18n.svelte.ts";

    let {children}: { children: Snippet } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

</script>

<NavSub width="11rem" paddingTop="9.5rem">
    <NavLinkSub href="/auth/v1/admin/config/policy">
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
</NavSub>

<ContentAdmin>
    {@render children()}
</ContentAdmin>
