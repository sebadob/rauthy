<script lang="ts">
    import type { EventLevel } from '#api/types/events.ts';
    import Options from '#lib/Options.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let { level = $bindable() }: { level: EventLevel } = $props();
    const ta = useI18nAdmin();
    const levels = ['info', 'notice', 'warning', 'critical'] as const;
    let options = $derived(levels.map(key => ta.events.levelFilters[key]));

    function select(label: string | number | undefined) {
        const next = levels.find(key => ta.events.levelFilters[key] === label);
        if (next) level = next;
    }
</script>

<Options
    ariaLabel={ta.events.eventLevel}
    {options}
    bind:value={() => ta.events.levelFilters[level], select}
    borderless
/>
