<script lang="ts">
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import A from '#lib/A.svelte';
    import AppVersion from '#lib/AppVersion.svelte';
    import Section from '#lib/Section.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let ta = useI18nAdmin();
    const configurationKeys = /^server\.swagger_ui_(enable|public)$/;
</script>

<ContentAdmin wide>
    <div class="about-content">
        <h2 class="admin-page-title">{ta.common.about}</h2>
        <div class="sections">
            <Section title="Rauthy">
                <AppVersion />
            </Section>
            <Section title={ta.nav.docs}>
                <p>
                    {ta.docs.book}
                    <A href="https://sebadob.github.io/rauthy/" target="_blank">Rauthy Book</A>
                </p>

                <p>
                    {ta.docs.openapi}
                    <A href="/auth/v1/docs/" target="_blank">Swagger UI</A>
                </p>

                <aside class="swagger-note" aria-label={ta.common.note}>
                    <p>{ta.docs.openapiNote}</p>
                    <p>
                        {#each ta.docs.openapiEnable.split(/(server\.swagger_ui_(?:enable|public))/g) as part}
                            {#if configurationKeys.test(part)}<code>{part}</code>{:else}{part}{/if}
                        {/each}
                    </p>
                </aside>

                <p>
                    {ta.docs.source}:
                    <A href="https://github.com/sebadob/rauthy" target="_blank">GitHub</A>
                </p>
            </Section>
        </div>
    </div>
</ContentAdmin>

<style>
    .about-content {
        max-width: 54rem;
    }

    .sections {
        display: grid;
        gap: var(--ui-space);
    }

    .sections p {
        margin: 0;
    }

    .swagger-note {
        display: grid;
        gap: 0.375rem;
        padding: 0.75rem 0.875rem;
        border-inline-start: 2px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-container);
        color: var(--ui-muted);
    }

    .swagger-note code {
        color: var(--ui-text);
        overflow-wrap: anywhere;
    }

    .sections :global(.ver) {
        margin: 0;
    }
</style>
