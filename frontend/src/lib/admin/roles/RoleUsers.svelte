<script lang="ts">
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useSession } from '#state/session.svelte.ts';
    import type { RoleResponse } from '#api/types/roles.ts';
    import AssignedUsers from '#lib/admin/users/AssignedUsers.svelte';

    let {
        role,
    }: {
        role: RoleResponse;
    } = $props();

    let ta = useI18nAdmin();
    let session = useSession('admin');

    // roles may never be touched by a group admin, only a full admin can remove a user from one
    let canDelete = $derived(session.isAdmin());
</script>

<AssignedUsers
    name={role.name}
    usersUrl={`/auth/v1/roles/${role.id}/users`}
    patchKey="roles"
    {canDelete}
    msgEmpty={ta.roles.users.empty}
    msgRemove={ta.roles.users.remove}
/>
