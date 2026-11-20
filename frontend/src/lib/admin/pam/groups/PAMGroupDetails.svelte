<script lang="ts">
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import IconUserGroup from '#icons/IconUserGroup.svelte';
    import type { PamGroupHostsCountResponse, PamGroupResponse } from '#api/types/pam.ts';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import IconInfo from '#icons/IconInfo.svelte';
    import { slide } from 'svelte/transition';
    import Button from '#lib/button/Button.svelte';
    import { fetchGet } from '#api/fetch.ts';

    let {
        group,
        onDelete,
    }: {
        group: PamGroupResponse;
        onDelete: () => void;
    } = $props();

    let ta = useI18nAdmin();

    let showInfo = $state(false);
    let hostsCount: number | undefined = $state();
    let canBeDeleted = $derived(
        group.typ === 'generic' ||
            group.typ === 'local' ||
            (group.typ === 'host' && hostsCount === 0),
    );

    $effect(() => {
        fetchHostsCount();
    });

    async function fetchHostsCount() {
        if (group.typ !== 'host') {
            hostsCount = 0;
            return;
        }

        const groupId = group.id;
        hostsCount = undefined;
        let url = `/auth/v1/pam/groups/${groupId}/hosts_count`;
        let res = await fetchGet<PamGroupHostsCountResponse>(url);
        if (groupId !== group.id) return;
        if (res.body) {
            hostsCount = res.body.count;
        } else {
            console.error(res.error);
        }
    }
</script>

<EntityHeader title={group.name} subtitle="PAM">
    {#snippet actions()}
        {#if canBeDeleted}
            {#key group.id}
                <EntityDelete
                    url={`/auth/v1/pam/groups/${group.id}`}
                    name={group.name}
                    message={ta.groups.delete1}
                    {onDelete}
                />
            {/key}
        {/if}
    {/snippet}
    <IconUserGroup width="28px" />
</EntityHeader>

<LabeledValue label="ID">
    {group.id}
</LabeledValue>
<LabeledValue label={ta.pam.groupName}>
    {group.name}
</LabeledValue>
<LabeledValue label={ta.pam.groupType}>
    {group.typ}
    <Button invisible onclick={() => (showInfo = !showInfo)}>
        <div class="info" data-selected={showInfo}>
            <IconInfo width="1.25rem" />
        </div>
    </Button>
</LabeledValue>

{#if showInfo}
    <div transition:slide={{ duration: 150 }}>
        <p>
            {#if group.name === 'wheel-rauthy'}
                {ta.pam.groupDescWheel}
            {:else if group.typ === 'host'}
                {ta.pam.groupDescHost}
            {:else if group.typ === 'generic'}
                {ta.pam.groupDescGeneric}
            {:else if group.typ === 'local'}
                {ta.pam.groupDescLocal}
            {:else if group.typ === 'user'}
                {ta.pam.groupDescUser}
            {/if}
        </p>
    </div>
{/if}

<style>
    .info {
        transform: translateY(0.25rem);
    }

    .info[data-selected='true'] {
        color: hsl(var(--action));
    }
</style>
