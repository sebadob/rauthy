<script lang="ts">
    import NavSub from "$lib5/nav/NavSub.svelte";
    import NavLinkSub from "$lib5/nav/NavLinkSub.svelte";
    import type {Snippet} from "svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";

    let {children}: { children: Snippet } = $props();

</script>

<NavSub width="10rem">
    <NavLinkSub href="/auth/v1/admin/config/policy">
        Password Policy
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/jwks">
        JWKS
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/argon2">
        Argon2 Params
    </NavLinkSub>
    <NavLinkSub href="/auth/v1/admin/config/encryption">
        Encryption
    </NavLinkSub>
</NavSub>

<ContentAdmin>
    {@render children()}
</ContentAdmin>
