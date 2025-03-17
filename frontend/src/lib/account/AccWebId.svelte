<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {fade, slide} from 'svelte/transition';
    import Switch from "$lib5/Switch.svelte";
    import {buildWebIdUri} from "$utils/helpers.ts";
    import {useI18n} from "$state/i18n.svelte.js";
    import type {WebIdResponse} from "$api/types/web_id.ts";
    import A from "$lib5/A.svelte";
    import InputArea from "$lib5/form/InputArea.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {fetchPut} from "$api/fetch.ts";

    let {
        webIdData = $bindable(),
    }: {
        webIdData: WebIdResponse,
    } = $props();

    let t = useI18n();

    const labelWidth = "14rem";

    let err = $state('');
    let success = $state(false);
    let expertMode = $state(!!webIdData.custom_triples);
    let webIdLink = buildWebIdUri(webIdData.user_id);

    async function onSubmit() {
        err = '';

        let res = await fetchPut(`/auth/v1/users/${webIdData.user_id}/webid/data`, webIdData);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
        }
    }

</script>

<div class="container">
    <p>{t.account.webIdDesc}</p>

    <p>
        <A href={webIdLink} target="_blank">
            {@html webIdLink.replace('/auth/', '/auth/<wbr/>')}
        </A>
    </p>

    <div class="switch">
        <Switch
                ariaLabel="E-Mail"
                bind:checked={webIdData.expose_email}
                {labelWidth}
        >
            E-Mail
        </Switch>
    </div>

    <div class="switch">
        <Switch
                ariaLabel={t.account.webIdExpertMode}
                bind:checked={expertMode}
                {labelWidth}
        >
            {t.account.webIdExpertMode}
        </Switch>
    </div>

    {#if expertMode}
        <div transition:slide={{duration: 150}}>
            <p>{t.account.webIdDescData}</p>
            <InputArea
                    placeholder="FOAF"
                    rows={15}
                    bind:value={webIdData.custom_triples}
            />
        </div>
    {/if}

    <div class="bottom">
        <Button onclick={onSubmit}>
            {t.common.save}
        </Button>

        {#if success}
            <div class="success" transition:fade>
                <IconCheck/>
            </div>
        {:else if err}
            <div class="err" transition:fade>
                {err}
            </div>
        {/if}
    </div>
</div>

<style>
    .bottom {
        margin-top: .5rem;
        display: flex;
        flex-direction: row;
        gap: .5rem;
    }

    .container {
        display: flex;
        flex-direction: column;
    }

    .err {
        margin: .2rem 0 0 .5rem;
        max-width: 23rem;
        color: hsl(var(--error));
    }

    .success {
        margin-top: .2rem;
    }

    .switch {
        margin: .5rem 0;
    }
</style>
