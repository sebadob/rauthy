<script lang="ts">
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import A from '#lib/A.svelte';
    import AppVersion from '#lib/AppVersion.svelte';
    import Section from '#lib/Section.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useTrigger } from '#state/callback.svelte.ts';

    let ta = useI18nAdmin();

    let ref: undefined | HTMLAnchorElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => ref?.focus());
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
                    <A bind:ref href="https://sebadob.github.io/rauthy/" target="_blank"
                        >Rauthy Book</A
                    >
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

    .sections :global(.ver) {
        margin: 0;
    }
</style>
