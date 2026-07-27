<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { fetchDelete } from '$api/fetch';
    import type { RoleResponse } from '$api/types/roles.ts';

    let {
        role,
        onSave,
    }: {
        role: RoleResponse;
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let isRauthyAdmin = $derived(role.name === 'rauthy_admin');

    async function onSubmit() {
        err = '';

        let res = await fetchDelete(`/auth/v1/roles/${role.id}`);
        if (res.error) {
            err = res.error.message;
        } else {
            onSave();
        }
    }
</script>

{#if isRauthyAdmin}
    <p>{@html ta.roles.adminNoMod}</p>
{:else}
    <p>{ta.roles.delete1}</p>

    <Button level={-1} onclick={onSubmit}>
        {t.common.delete}
    </Button>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
{/if}
