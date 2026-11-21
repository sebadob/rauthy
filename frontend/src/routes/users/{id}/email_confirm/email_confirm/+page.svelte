<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import PublicPage from '#lib/PublicPage.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Template from '#lib/Template.svelte';
    import { TPL_EMAIL_NEW, TPL_EMAIL_OLD } from '#utils/constants.js';

    let t = useI18n();

    let emailOld = $state('old@mail.org');
    let emailNew = $state('new@mail.org');
</script>

<svelte:head>
    <title>{t.emailChange.title || 'E-Mail Change Confirm'}</title>
</svelte:head>

<Template id={TPL_EMAIL_OLD} bind:value={emailOld} />
<Template id={TPL_EMAIL_NEW} bind:value={emailNew} />

<PublicPage>
    <div class="public-card">
        <h1>{t.emailChange.title}</h1>
        <p>
            {t.emailChange.textChanged}:<br />
            <b>{emailOld}</b>
            {t.emailChange.to}
            <b>{emailNew}</b>
        </p>
        <p>{t.emailChange.textLogin}</p>
        <div class="btn">
            <Button onclick={() => window.location.replace('/auth/v1/account')}>
                {t.authorize.login}
            </Button>
        </div>
    </div>
</PublicPage>

<style>
    p {
        margin: 0.5rem 0;
    }

    .btn {
        margin-left: -5px;
        width: 7.5rem;
    }
</style>
