<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { fetchDelete } from '$api/fetch';

    let {
        userId,
    }: {
        userId: string;
    } = $props();

    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);

    async function onSubmit() {
        err = '';

        let res = await fetchDelete(`/auth/v1/sessions/${userId}`);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
        }
    }
</script>

<p>{ta.users.forceLogout}</p>

<div class="flex gap-05">
    <Button level={-1} onclick={onSubmit}>Logout</Button>
    {#if success}
        <IconCheck />
    {/if}
</div>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<style>
</style>
