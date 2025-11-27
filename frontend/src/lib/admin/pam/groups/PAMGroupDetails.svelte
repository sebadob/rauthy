<script lang="ts">
    import type { PamGroupHostsCountResponse, PamGroupResponse } from '$api/types/pam';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import IconInfo from '$icons/IconInfo.svelte';
    import { slide } from 'svelte/transition';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { fetchDelete, fetchGet } from '$api/fetch';

    let {
        group,
        onDelete,
    }: {
        group: PamGroupResponse;
        onDelete: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let showInfo = $state(false);
    let hostsCount = $state(0);
    let canBeDeleted = $derived(
        group.typ === 'generic' ||
            group.typ === 'local' ||
            (group.typ === 'host' && hostsCount === 0),
    );

    $effect(() => {
        fetchHostsCount();
    });

    async function deleteGroup() {
        let url = `/auth/v1/pam/groups/${group.id}`;
        let res = await fetchDelete(url);
        if (res.error) {
            console.error(res.error);
        } else {
            onDelete();
        }
    }

    async function fetchHostsCount() {
        if (group.typ !== 'host') {
            hostsCount = 0;
            return;
        }

        let url = `/auth/v1/pam/groups/${group.id}/hosts_count`;
        let res = await fetchGet<PamGroupHostsCountResponse>(url);
        if (res.body) {
            hostsCount = res.body.count;
        } else {
            console.error(res.error);
        }
    }
</script>

<h1>{group.name}</h1>

<LabeledValue label="ID">
    {group.id}
</LabeledValue>
<LabeledValue label={ta.pam.groupName}>
    {group.name}
</LabeledValue>
<LabeledValue label={ta.pam.groupType}>
    {group.typ}
    <Button
        invisible
        onclick={() => (showInfo = !showInfo)}
    >
        <div
            class="info"
            data-selected={showInfo}
        >
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

{#if canBeDeleted}
    <div class="deleteBtn">
        <Button
            ariaLabel={t.common.delete}
            level={-1}
            onclick={deleteGroup}
        >
            {t.common.delete}
        </Button>
    </div>
{/if}

<style>
    h1 {
        margin-top: 0;
    }

    .deleteBtn {
        margin: 1rem 0;
    }

    .info {
        transform: translateY(0.25rem);
    }

    .info[data-selected='true'] {
        color: hsl(var(--action));
    }
</style>
