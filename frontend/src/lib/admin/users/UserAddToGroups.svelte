<script lang="ts">
    import { fetchPatch } from '$api/fetch';
    import type { GroupResponse } from '$api/types/groups.ts';
    import type { PatchOp } from '$api/types/generic';
    import type { SelectItem } from '$lib5/select_list/props.ts';
    import SelectList from '$lib5/select_list/SelectList.svelte';
    import Button from '$lib5/button/Button.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useSession } from '$state/session.svelte';

    let {
        userId,
        groups,
        onAdded,
    }: {
        userId: string;
        groups: GroupResponse[];
        // called after the user has been added to a managed group, so the parent can re-fetch
        // and switch over to the now-visible user details
        onAdded: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();
    let session = useSession('admin');

    let err = $state('');
    let success = $state(false);

    // Only the groups this admin manages can be added. The user may be a member of other
    // groups, but those stay hidden here and are preserved by the backend on the PATCH (#1538).
    let managed = $derived(session.managedGroups(groups.map(g => g.name)));
    let items: SelectItem[] = $state([]);
    $effect(() => {
        items = managed.map(name => ({ name, selected: false }));
    });

    let anySelected = $derived(items.some(i => i.selected));

    async function onSubmit() {
        err = '';
        let selected = items.filter(i => i.selected).map(i => i.name);
        if (selected.length === 0) {
            return;
        }

        // a very specific, groups-only PATCH: the admin never saw the user's other groups, so
        // it only sends the in-scope ones it wants to add. The backend merges these with the
        // memberships outside the admin's scope instead of replacing the whole set.
        let payload: PatchOp = {
            put: [{ key: 'groups', value: selected }],
            del: [],
        };
        let res = await fetchPatch(`/auth/v1/users/${userId}`, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            onAdded();
        }
    }
</script>

<div class="container">
    <h3>{ta.users.groupAdmin.notManagedTitle}</h3>
    <p>{ta.users.groupAdmin.notManagedDesc}</p>

    <SelectList bind:items>
        {t.account.groups.replaceAll(',', ' ')}
    </SelectList>

    <div class="flex gap-05">
        <Button onclick={onSubmit} isDisabled={!anySelected}>
            {ta.users.groupAdmin.addToGroups}
        </Button>
        {#if success}
            <IconCheck />
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 1rem 0.5rem;
    }
</style>
