<script lang="ts">
    import UserConsents from '#lib/consents/UserConsents.svelte';
    import { useI18n } from '#state/i18n.svelte.js';
    import type { UserResponse } from '#api/types/user.ts';

    let {
        user,
    }: {
        user: UserResponse;
    } = $props();

    let t = useI18n();
</script>

<div class="ui-section-stack">
    <h2 class="ui-page-title">{t.account.privacy.connectedApps}</h2>
    <UserConsents
        userId={user.id}
        description={t.account.privacy.connectedAppsDesc}
        empty={t.account.privacy.noConnectedApps}
        revokeDesc={t.account.privacy.revokeDesc}
        revokeAllDesc={t.account.privacy.revokeAllDesc}
        sessionsWarning={t.account.privacy.logoutWarning}
        loginState="account"
    />
</div>

<style>
    h2 {
        margin: 0;
    }
</style>
