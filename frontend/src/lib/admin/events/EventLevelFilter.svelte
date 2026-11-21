<script lang="ts">
    import { EVENT_LEVELS, type EventLevel } from '#api/types/events.ts';
    import IconChevronDown from '#icons/IconChevronDown.svelte';
    import Popover from '#lib/Popover.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let { levels = $bindable(), field }: { levels: EventLevel[]; field?: boolean } = $props();
    let ta = useI18nAdmin();
    let all = $derived(levels.length === EVENT_LEVELS.length);

    function toggle(level: EventLevel, checked: boolean) {
        levels = EVENT_LEVELS.filter(value => (value === level ? checked : levels.includes(value)));
    }
</script>

<div class="level-filter" class:field>
    <Popover ariaLabel={ta.events.eventLevel} btnInvisible anchored placement="below">
        {#snippet button()}
            <span class="selected">
                {#if all}
                    {ta.events.allLevels}
                {:else if levels.length === 1}
                    <span class="level">{levels[0]}</span>
                {:else}
                    {levels.length} / {EVENT_LEVELS.length}
                {/if}
                <IconChevronDown width=".8rem" />
            </span>
        {/snippet}

        <div class="levels" role="group" aria-label={ta.events.eventLevel}>
            <label class="all">
                <input
                    type="checkbox"
                    checked={all}
                    indeterminate={!all && levels.length > 0}
                    onchange={event =>
                        (levels = event.currentTarget.checked ? [...EVENT_LEVELS] : [])}
                />
                {ta.events.allLevels}
            </label>
            {#each EVENT_LEVELS as level (level)}
                <label>
                    <input
                        type="checkbox"
                        checked={levels.includes(level)}
                        onchange={event => toggle(level, event.currentTarget.checked)}
                    />
                    <span class="level">{level}</span>
                </label>
            {/each}
        </div>
    </Popover>
</div>

<style>
    .selected,
    .levels label {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
        font-weight: 400;
        line-height: var(--ui-control-line-height);
    }

    .level {
        text-transform: capitalize;
    }
    .field {
        --ui-button-height: var(--ui-field-height);
        --ui-button-min-height: var(--ui-field-height);
        --ui-button-padding: 0 14px;
    }
    .field > :global(div > .admin-control[data-variant]) {
        width: 100%;
        border: 1px solid var(--ui-outline);
        background: var(--ui-surface);
    }
    .field > :global(div > .admin-control > .children),
    .field .selected {
        width: 100%;
        justify-content: space-between;
    }

    .levels {
        min-width: 9rem;
        max-width: calc(100dvw - 2rem);
        padding: 0.25rem;
    }

    .levels label {
        min-height: 2.25rem;
        padding: 0.25rem 0.5rem;
        border-radius: var(--border-radius);
        cursor: pointer;
    }

    .levels label:hover {
        background: var(--ui-hover);
    }

    .levels .all {
        border-bottom: 1px solid var(--ui-outline);
        border-bottom-left-radius: 0;
        border-bottom-right-radius: 0;
    }

    input[type='checkbox'] {
        width: 1.125rem;
        height: 1.125rem;
        margin: 0;
        flex-shrink: 0;
        accent-color: var(--ui-primary);
    }
</style>
