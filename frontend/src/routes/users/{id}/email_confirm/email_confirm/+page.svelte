<script>
    import {onMount} from "svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Button from "$lib/Button.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";

    let t = useI18n();
    let emailOld = $state('old@mail.org');
    let emailNew = $state('new@mail.org');

    onMount(async () => {
        const data = document.getElementsByName('rauthy-data')[0].id;
        const arr = [];
        data.split(',').forEach(i => arr.push(i));
        emailOld = arr[0];
        emailNew = arr[1];
    })

</script>

<svelte:head>
    <title>{t.emailChange.title || 'E-Mail Change Confirm'}</title>
</svelte:head>

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
                <Button on:click={() => window.location.replace('/auth/v1/account')}>
                    {t.authorize.login}
                </Button>
            </div>
        </div>
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
