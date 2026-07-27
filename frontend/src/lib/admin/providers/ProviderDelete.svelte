<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { onMount } from 'svelte';
    import type { ProviderLinkedUserResponse, ProviderResponse } from '$api/types/auth_provider.ts';
    import { fetchDelete, fetchGet } from '$api/fetch';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import Expandable from '$lib5/Expandable.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useI18n } from '$state/i18n.svelte';

    let {
        provider,
        onSave = $bindable(),
    }: {
        provider: ProviderResponse;
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let isLoading = $state(true);
    let err = $state('');
    let forceDelete = $state(false);
    let linkedUsers: ProviderLinkedUserResponse[] = $state([]);

    onMount(async () => {
        let res = await fetchGet(`/auth/v1/providers/${provider.id}/delete_safe`);

        // If there are any linked users, the backend will return 406 instead of 200
        // to make it as clear as possible, that it is NOT safe to delete.
        if (res.status === 406 && res.error) {
            linkedUsers = res.error as unknown as ProviderLinkedUserResponse[];
        }

        isLoading = false;
    });

    async function onSubmit() {
        err = '';
        isLoading = true;

        let res = await fetchDelete(`/auth/v1/providers/${provider.id}`);
        if (res.error) {
            err = res.error.message;
        } else {
            onSave();
        }

        isLoading = false;
    }
</script>

<div class="container">
    {#if linkedUsers.length > 0}
        <p class="err"><b>{ta.providers.delete.isInUse1}</b></p>
        <p>{ta.providers.delete.isInUse2}</p>

        <Expandable>
            {#snippet summary()}
                {ta.providers.delete.linkedUsers}
            {/snippet}
            {#snippet details()}
                {#each linkedUsers as user (user.id)}
                    <div class="user">
                        {user.email}
                        <span class="muted font-mono"> / {user.id}</span>
                    </div>
                {/each}
            {/snippet}
        </Expandable>

        <p>{ta.providers.delete.areYouSure}</p>
        <InputCheckbox ariaLabel={ta.providers.delete.forceDelete} bind:checked={forceDelete}>
            <div class="forceDelete">
                {ta.providers.delete.forceDelete}
            </div>
        </InputCheckbox>

        {#if forceDelete}
            <Button level={-1} onclick={onSubmit} {isLoading}>
                {t.common.delete}
            </Button>
        {/if}
    {:else}
        <p>{ta.providers.delete.areYouSure}</p>
        <Button level={-1} onclick={onSubmit} {isLoading}>
            {t.common.delete}
        </Button>
    {/if}
    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0 10px 20px 10px;
    }

    .forceDelete {
        margin-bottom: 0.5rem;
        color: hsl(var(--error));
        font-weight: bold;
    }

    .muted {
        color: hsla(var(--text) / 0.6);
    }

    .user {
        margin: 0.15rem 0;
    }
</style>
