<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { fetchDelete } from '$api/fetch';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useSession } from '$state/session.svelte';

    let {
        userId,
        onSave,
    }: {
        userId: string;
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();
    let session = useSession('admin');

    let err = $state('');

    async function onSubmit() {
        err = '';

        let res = await fetchDelete(`/auth/v1/users/${userId}`);
        if (res.error) {
            err = res.error.message;
        } else {
            onSave();
        }
    }
</script>

{#if session.get()?.user_id === userId}
    <p class="err">
        <b>
            {ta.users.antiLockout.rule}:
            {ta.users.antiLockout.delete}
        </b>
    </p>
{:else}
    <p>{ta.users.deleteUser}</p>
    <Button level={-1} onclick={onSubmit}>
        {t.common.delete}
    </Button>
{/if}

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<style>
</style>
