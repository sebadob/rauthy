<script lang="ts">
    import LangSelector from "$lib5/LangSelector.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import A from "$lib/A.svelte";

    let t = useI18n();

</script>

<svelte:head>
    <title>{t.userRevoke.title || 'Login Revoke'}</title>
</svelte:head>

<Main>
    <ContentCenter>
        <div class="container">
            <h1>{t.userRevoke.title}</h1>
            <p>{t.userRevoke.desc1}</p>
            <p class="err">{t.userRevoke.desc2}</p>

            <A href="/auth/v1/account">Account</A>
        </div>

        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>

<style>
    p {
        margin: .5rem 0;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
</style>
