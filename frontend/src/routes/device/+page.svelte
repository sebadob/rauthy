<script>
    import {onMount} from "svelte";
    import {getSessionInfo} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import {redirectToLogin} from "../../utils/helpers.js";
    import BrowserCheck from "../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";

    let t;
    let sessionInfo;

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            sessionInfo = await res.json();
        } else {
            redirectToLogin('device');
        }
    });
</script>

<svelte:head>
    <title>{t?.title || 'Device Authorization'}</title>
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="device">
        {#if !sessionInfo}
            <Loading/>
        {:else}
            {sessionInfo.user_id}
        {/if}

        <LangSelector absolute/>
    </WithI18n>
</BrowserCheck>
