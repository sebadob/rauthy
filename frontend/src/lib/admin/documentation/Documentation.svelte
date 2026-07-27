<script lang="ts">
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import A from '$lib5/A.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte.js';
    import { useTrigger } from '$state/callback.svelte';

    let ta = useI18nAdmin();

    let ref: undefined | HTMLAnchorElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => ref?.focus());
</script>

<ContentAdmin>
    <h1>{ta.nav.docs}</h1>

    <p>
        {ta.docs.book}
        <A bind:ref href="https://sebadob.github.io/rauthy/" target="_blank">Rauthy Book</A>
    </p>

    <p>
        {ta.docs.openapi}
        <A href="/auth/v1/docs/" target="_blank">Swagger UI</A>
        <br />
        <b>{ta.common.note}:</b>
        {ta.docs.openapiNote}
    </p>

    <p>
        {ta.docs.source}:
        <A href="https://github.com/sebadob/rauthy" target="_blank">GitHub</A>
    </p>
</ContentAdmin>
