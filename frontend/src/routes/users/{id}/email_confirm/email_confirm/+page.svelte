<script>
    import {onMount} from "svelte";
    import BrowserCheck from "../../../../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import Button from "$lib/Button.svelte";

    let t;
    let emailOld = 'old@mail.org';
    let emailNew = 'new@mail.org';

    onMount(async () => {
        const data = document.getElementsByName('rauthy-data')[0].id;
        const arr = [];
        data.split(',').forEach(i => arr.push(i));
        emailOld = arr[0];
        emailNew = arr[1];
    })

</script>

<svelte:head>
    <title>{t?.title || 'E-Mail Change Confirm'}</title>
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="emailChangeConfirm">
        <div class="container">
            <h1>{t.title}</h1>
            <p>
                {t.text_changed}:<br/>
                <b>{emailOld}</b>
                {t.to}
                <b>{emailNew}</b>
            </p>
            <p>{t.text_login}</p>
            <div class="btn">
                <Button on:click={() => window.location.replace('/auth/v1/account')}>
                    Account Login
                </Button>
            </div>
        </div>
        <LangSelector absolute/>
    </WithI18n>
</BrowserCheck>

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
