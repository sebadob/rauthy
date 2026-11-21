<script lang="ts">
    import PublicPage from '#lib/PublicPage.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import A from '#lib/A.svelte';

    let t = useI18n();
</script>

<svelte:head>
    <title>{t.userRevoke.title || 'Login Revoke'}</title>
</svelte:head>

<PublicPage>
    <div class="public-card">
        <h1>{t.userRevoke.title}</h1>
        <p>{t.userRevoke.desc1}</p>
        <p class="err">{t.userRevoke.desc2}</p>

        <A href="/auth/v1/account" reload>Account</A>
    </div>
</PublicPage>

<style>
    p {
        margin: 0.5rem 0;
    }
</style>
