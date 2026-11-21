<script lang="ts">
    import PublicPage from '#lib/PublicPage.svelte';
    import { useI18n } from '#state/i18n.svelte.js';
    import Template from '#lib/Template.svelte';
    import { TPL_ERROR_TEXT, TPL_STATUS_CODE } from '#utils/constants.ts';
    import Expandable from '#lib/Expandable.svelte';

    let t = useI18n();

    let statusCode = $state('');
    let errorText = $state('');
    let errorDetails = $state('');
</script>

<Template id={TPL_STATUS_CODE} bind:value={statusCode} />
<Template id={TPL_ERROR_TEXT} bind:value={errorText} />

<PublicPage>
    <div class="public-card ui-section-stack">
        <h1>{statusCode}</h1>
        <p>{errorText}</p>

        {#if errorDetails}
            <Expandable>
                {#snippet summary()}
                    {t.error.details}
                {/snippet}
                {#snippet details()}
                    {errorDetails}
                {/snippet}
            </Expandable>
        {/if}
    </div>
</PublicPage>
