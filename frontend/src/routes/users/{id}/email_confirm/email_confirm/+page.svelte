<script lang="ts">
    import LangSelector from "$lib5/LangSelector.svelte";
    import Button from "$lib5/button/Button.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {TPL_EMAIL_NEW, TPL_EMAIL_OLD} from "$utils/constants.js";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";

    let t = useI18n();

    let emailOld = $state('old@mail.org');
    let emailNew = $state('new@mail.org');

</script>

<svelte:head>
    <title>{t.emailChange.title || 'E-Mail Change Confirm'}</title>
</svelte:head>

<Template id={TPL_EMAIL_OLD} bind:value={emailOld}/>
<Template id={TPL_EMAIL_NEW} bind:value={emailNew}/>

<Main>
    <ContentCenter>
        <div class="container">
            <h1>{t.emailChange.title}</h1>
            <p>
                {t.emailChange.textChanged}:<br/>
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

        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>

<style>
    p {
        margin: .5rem 0;
    }

    .btn {
        margin-left: -5px;
        width: 7.5rem;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
</style>
