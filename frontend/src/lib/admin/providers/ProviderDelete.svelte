<script lang="ts">
    import type { ProviderLinkedUserResponse, ProviderResponse } from '#api/types/auth_provider.ts';
    import { fetchGet } from '#api/fetch.ts';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import Expandable from '#lib/Expandable.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let { provider, onSave }: { provider: ProviderResponse; onSave: () => void } = $props();
    const ta = useI18nAdmin();

    let loading = $state(false);
    let error = $state('');
    let forceDelete = $state(false);
    let linkedUsers: ProviderLinkedUserResponse[] = $state([]);

    async function checkLinkedUsers() {
        loading = true;
        error = '';
        forceDelete = false;
        linkedUsers = [];
        const res = await fetchGet(`/auth/v1/providers/${provider.id}/delete_safe`);
        if (res.status === 406 && Array.isArray(res.error)) {
            linkedUsers = res.error;
        } else if (res.error) {
            error = res.error.message;
        }
        loading = false;
    }
</script>

<EntityDelete
    url={`/auth/v1/providers/${provider.id}`}
    name={provider.name}
    message={ta.providers.delete.areYouSure}
    onOpen={checkLinkedUsers}
    confirmDisabled={loading || !!error || (linkedUsers.length > 0 && !forceDelete)}
    onDelete={onSave}
>
    {#snippet details()}
        {#if error}<p class="err" role="alert">{error}</p>{/if}
        {#if linkedUsers.length > 0}
            <p class="err">{ta.providers.delete.isInUse1}</p>
            <p>{ta.providers.delete.isInUse2}</p>
            <Expandable>
                {#snippet summary()}{ta.providers.delete.linkedUsers}{/snippet}
                {#snippet details()}
                    {#each linkedUsers as user (user.id)}
                        <div>{user.email} <span class="font-mono">/ {user.id}</span></div>
                    {/each}
                {/snippet}
            </Expandable>
            <InputCheckbox ariaLabel={ta.providers.delete.forceDelete} bind:checked={forceDelete}>
                {ta.providers.delete.forceDelete}
            </InputCheckbox>
        {/if}
    {/snippet}
</EntityDelete>
