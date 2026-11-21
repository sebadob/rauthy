<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import Section from '#lib/Section.svelte';
    import { fade, slide } from 'svelte/transition';
    import Switch from '#lib/Switch.svelte';
    import { buildWebIdUri } from '#utils/helpers.ts';
    import { useI18n } from '#state/i18n.svelte.js';
    import type { WebIdResponse } from '#api/types/web_id.ts';
    import A from '#lib/A.svelte';
    import InputArea from '#lib/form/InputArea.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import { fetchPut } from '#api/fetch.ts';

    let {
        webIdData = $bindable(),
    }: {
        webIdData: WebIdResponse;
    } = $props();

    let t = useI18n();

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

<div class="ui-section-stack">
    <Section title="WebID">
        <p class="ui-muted-copy">{t.account.webIdDesc}</p>
        <p class="profile-link">
            <A href={webIdLink} target="_blank">{webIdLink}</A>
        </p>
        <Switch ariaLabel={t.common.email} bind:checked={webIdData.expose_email}>
            {t.common.email}
        </Switch>
    </Section>

    <Section title="FOAF">
        <Switch ariaLabel={t.account.webIdExpertMode} bind:checked={expertMode}>
            {t.account.webIdExpertMode}
        </Switch>
        {#if expertMode}
            <div transition:slide={{ duration: 150 }}>
                <p class="ui-muted-copy">{t.account.webIdDescData}</p>
                <InputArea label="FOAF" rows={12} fontMono bind:value={webIdData.custom_triples} />
            </div>
        {/if}
    </Section>

    <div class="bottom">
        <Button onclick={onSubmit}>{t.common.save}</Button>
        {#if success}
            <span class="success" transition:fade><IconCheck /></span>
        {:else if err}
            <span class="err" role="alert" transition:fade>{err}</span>
        {/if}
    </div>
</div>

<style>
    .bottom {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .profile-link {
        overflow-wrap: anywhere;
    }

    .success {
        color: var(--ui-primary);
    }
</style>
