<script lang="ts">
    import UserConsents from '#lib/consents/UserConsents.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useSession } from '#state/session.svelte.ts';

    let {
        userId,
    }: {
        userId: string;
    } = $props();

    let ta = useI18nAdmin();
    let session = useSession('admin');
</script>

{#key userId}
    <UserConsents
        {userId}
        description={ta.users.consents.desc}
        empty={ta.users.consents.none}
        revokeDesc={ta.users.consents.revokeDesc}
        revokeAllDesc={ta.users.consents.revokeAllDesc}
        sessionsWarning={ta.users.consents.sessionsWarning}
        loginState={session.get()?.user_id === userId ? 'admin' : undefined}
    />
{/key}
