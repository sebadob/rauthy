<script lang="ts">
    import Documentation from '#lib/admin/documentation/Documentation.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    const ta = useI18nAdmin();
</script>

<svelte:head>
    <title>Rauthy · {ta.common.about}</title>
</svelte:head>

<Documentation />
