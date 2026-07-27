<script lang="ts">
    import { genKey } from '$utils/helpers';
    import IconTrash from '$icons/IconTrash.svelte';
    import Button from '$lib/button/Button.svelte';

    let {
        providerId,
        onDelete,
    }: {
        providerId: string;
        onDelete: () => void;
    } = $props();

    let exists = $state(false);
</script>

<div class="container">
    <img
        src={`/auth/v1/providers/${providerId}/img?${genKey()}}`}
        width={20}
        height={20}
        alt="-"
        onload={() => (exists = true)}
    />
    {#if exists}
        <Button invisible onclick={onDelete}>
            <IconTrash />
        </Button>
    {/if}
</div>

<style>
    .container {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
</style>
