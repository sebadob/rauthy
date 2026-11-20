<script lang="ts">
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useSession } from '#state/session.svelte.ts';
    import type { GroupResponse } from '#api/types/groups.ts';
    import AssignedUsers from '#lib/admin/users/AssignedUsers.svelte';

    let {
        group,
    }: {
        group: GroupResponse;
    } = $props();

    let ta = useI18nAdmin();
    let session = useSession('admin');

    // removing a user from a group is a user update: a group admin may only do that for groups
    // it manages, a full admin for all of them
    let canDelete = $derived(session.isAdmin() || session.managesGroup(group.name));
</script>

<AssignedUsers
    name={group.name}
    usersUrl={`/auth/v1/groups/${group.id}/users`}
    patchKey="groups"
    {canDelete}
    msgEmpty={ta.groups.users.empty}
    msgRemove={ta.groups.users.remove}
/>
