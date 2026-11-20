<script lang="ts">
    import { eventColor } from '#utils/helpers.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    let { wide = false }: { wide?: boolean } = $props();
    const levels = ['info', 'notice', 'warning', 'critical'] as const;
    let t = useI18n();
</script>
<details class="admin-event-legend">
    <summary>{t.common.legend}</summary>
    <ul class:wide>
        {#each levels as level}<li>
                <span class="admin-legend-dot" style:background={eventColor(level)}></span>{level}
            </li>{/each}
    </ul>
</details>

<style>
    .admin-event-legend {
        flex-shrink: 0;
        padding: 12px 16px;
        border-top: 1px solid var(--ui-outline);
        background: var(--ui-surface);
    }
    .admin-event-legend :global(summary) {
        font-size: 12px;
        cursor: pointer;
    }
    .admin-event-legend :global(ul) {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 8px;
        margin: 12px 0 0;
        padding: 0;
    }
    .admin-event-legend :global(li) {
        display: flex;
        align-items: center;
        gap: 8px;
        margin: 0;
        padding: 0;
        font-size: 12px;
        text-transform: capitalize;
    }
    .admin-legend-dot {
        width: 7px;
        height: 7px;
        border-radius: 50%;
    }
</style>
