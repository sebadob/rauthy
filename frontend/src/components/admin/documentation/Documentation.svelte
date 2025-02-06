<script lang="ts">
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import A from "$lib5/A.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";

    let ta = useI18nAdmin();
</script>

<ContentAdmin>
    <h1>{ta.nav.docs}</h1>

    <p>
        {ta.docs.book}
        <A href="https://sebadob.github.io/rauthy/" target="_blank">Rauthy Book</A>
    </p>

    <p>
        {ta.docs.openapi}
        <A href="/docs/v1/swagger-ui/" target="_blank">Swagger UI</A>
        <br/>
        <b>{ta.common.note}:</b>
        {ta.docs.openapiNote}
    </p>

    <p>
        {ta.docs.source}:
        <A href="https://github.com/sebadob/rauthy" target="_blank">Github</A>
    </p>
</ContentAdmin>
