<script lang="ts">
    import { slide } from 'svelte/transition';
    import { tick } from 'svelte';
    import { CalendarDate, getDayOfWeek, parseDate } from '@internationalized/date';
    import { dayToString, fmtDateInput, getWeeksInMonth } from '#utils/form.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Popover from '#lib/Popover.svelte';
    import Options from '#lib/Options.svelte';
    import IconCalendar from '#icons/IconCalendar.svelte';
    import type { PropsInputDate } from '#lib/form/props.ts';

    const fieldId = $props.id();
    let {
        id = fieldId,
        name,
        value = $bindable(fmtDateInput()),
        label = '',
        errMsg = '',
        disabled,
        min = '1900-01-01',
        max = '2100-01-01',
        required,
        openTop = false,

        onEnter,
        onLeft,
        onRight,
        onUp,
        onDown,
    }: PropsInputDate = $props();

    const t = useI18n();
    const today = parseDate(fmtDateInput());
    const todayStr = today.toString();

    let isErr = $state(false);
    let trigger: HTMLButtonElement | undefined = $state();
    let close: (() => void) | undefined = $state();
    let grid: HTMLDivElement | undefined = $state();
    let cursor = $state(today);

    const first = $derived(parseDate(min));
    const last = $derived(parseDate(max));
    const months = $derived(
        t.common.months.slice(
            cursor.year === first.year ? first.month - 1 : 0,
            cursor.year === last.year ? last.month : 12,
        ),
    );
    const years = $derived(
        Array.from({ length: last.year - first.year + 1 }, (_, i) => first.year + i),
    );
    const displayedYear = $derived(cursor.year);
    const displayedMonth = $derived(cursor.month);
    const weeks = $derived(
        getWeeksInMonth(new CalendarDate(displayedYear, displayedMonth, 1), t.lang),
    );
    const dateFormat = $derived(
        new Intl.DateTimeFormat(t.lang, { dateStyle: 'full', timeZone: 'UTC' }),
    );
    const weekdayFormat = $derived(
        new Intl.DateTimeFormat(t.lang, { weekday: 'short', timeZone: 'UTC' }),
    );

    function withinBounds(date: CalendarDate) {
        if (date.compare(first) < 0) return first;
        if (date.compare(last) > 0) return last;
        return date;
    }

    async function focusCursor() {
        await tick();
        grid?.querySelector<HTMLButtonElement>('button[tabindex="0"]')?.focus();
    }

    function onToggle(state: 'open' | 'closed') {
        if (state === 'open') {
            cursor = withinBounds(value ? parseDate(value) : today);
            void focusCursor();
        }
    }

    function dismiss() {
        close?.();
        trigger?.focus({ preventScroll: true });
    }

    function select(date: string) {
        value = date;
        isErr = false;
        dismiss();
    }

    function onDayKeydown(event: KeyboardEvent) {
        let next: CalendarDate;
        switch (event.key) {
            case 'ArrowLeft':
                next = cursor.subtract({ days: 1 });
                break;
            case 'ArrowRight':
                next = cursor.add({ days: 1 });
                break;
            case 'ArrowUp':
                next = cursor.subtract({ weeks: 1 });
                break;
            case 'ArrowDown':
                next = cursor.add({ weeks: 1 });
                break;
            case 'Home':
                next = cursor.subtract({ days: getDayOfWeek(cursor, t.lang) });
                break;
            case 'End':
                next = cursor.add({ days: 6 - getDayOfWeek(cursor, t.lang) });
                break;
            case 'PageUp':
                next = cursor.subtract(event.shiftKey ? { years: 1 } : { months: 1 });
                break;
            case 'PageDown':
                next = cursor.add(event.shiftKey ? { years: 1 } : { months: 1 });
                break;
            case 'Escape':
                event.preventDefault();
                event.stopPropagation();
                dismiss();
                return;
            default:
                return;
        }
        event.preventDefault();
        cursor = withinBounds(next);
        void focusCursor();
    }

    function onKeyDownInput(ev: KeyboardEvent) {
        switch (ev.code) {
            case 'Enter':
                onEnter?.();
                break;
            case 'ArrowUp':
                onUp?.();
                break;
            case 'ArrowDown':
                onDown?.();
                break;
            case 'ArrowLeft':
                onLeft?.();
                break;
            case 'ArrowRight':
                onRight?.();
                break;
        }
    }

    function oninvalid(event: Event) {
        if (errMsg) {
            event.preventDefault();
        }
        isErr = true;
    }
</script>

<div class="admin-date-field">
    <div class="label admin-field-label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
    </div>

    <div class="date-control">
        <input
            class="ui-text-input"
            type="date"
            {id}
            {name}
            title={label}
            aria-label={label}
            bind:value
            {disabled}
            {required}
            aria-invalid={isErr}
            {min}
            {max}
            step="any"
            {oninvalid}
            onkeydown={onKeyDownInput}
            onblur={event => (isErr = !event.currentTarget.validity.valid)}
        />
        <div class="indicator">
            <Popover
                bind:ref={trigger}
                bind:close
                ariaLabel={label || t.common.month}
                fitViewport
                placement={openTop ? 'above' : 'below'}
                {onToggle}
                btnInvisible
                btnDisabled={disabled}
                lazy
            >
                {#snippet button()}
                    <IconCalendar color="currentColor" width="1.2rem" />
                {/snippet}

                <div class="calendar">
                    <div class="selectors">
                        <Options
                            ariaLabel={t.common.month}
                            options={months}
                            bind:value={
                                () => t.common.months[cursor.month - 1],
                                month => {
                                    cursor = withinBounds(
                                        cursor.set({
                                            month: t.common.months.indexOf(String(month)) + 1,
                                        }),
                                    );
                                }
                            }
                            maxHeight="13rem"
                            hideScrollbar
                            borderless
                        />
                        <Options
                            ariaLabel={t.common.year}
                            options={years}
                            bind:value={
                                () => cursor.year,
                                year => {
                                    cursor = withinBounds(cursor.set({ year: Number(year) }));
                                }
                            }
                            maxHeight="13rem"
                            hideScrollbar
                            borderless
                        />
                    </div>
                    <div class="visually-hidden" aria-live="polite">
                        {t.common.months[cursor.month - 1]}
                        {cursor.year}
                    </div>
                    <div bind:this={grid} role="grid" aria-label={label || t.common.month}>
                        <div class="week" role="row">
                            {#each weeks[0].days as day (dayToString(day))}
                                <span role="columnheader">
                                    {weekdayFormat.format(
                                        parseDate(dayToString(day)).toDate('UTC'),
                                    )}
                                </span>
                            {/each}
                        </div>
                        {#each weeks as week (dayToString(week.days[0]))}
                            <div class="week" role="row">
                                {#each week.days as day (dayToString(day))}
                                    {@const date = dayToString(day)}
                                    {@const current = parseDate(date)}
                                    {@const available =
                                        day.month === cursor.month &&
                                        current.compare(first) >= 0 &&
                                        current.compare(last) <= 0}
                                    <div role="gridcell" aria-selected={date === value}>
                                        <button
                                            type="button"
                                            class="day"
                                            disabled={!available}
                                            tabindex={date === cursor.toString() ? 0 : -1}
                                            aria-label={dateFormat.format(current.toDate('UTC'))}
                                            aria-current={date === todayStr ? 'date' : undefined}
                                            onclick={() => select(date)}
                                            onkeydown={onDayKeydown}
                                        >
                                            <time datetime={date}>{day.day}</time>
                                        </button>
                                    </div>
                                {/each}
                            </div>
                        {/each}
                    </div>
                </div>
            </Popover>
        </div>
    </div>

    {#if isErr && errMsg}
        <div class="error" transition:slide={{ duration: 150 }}>
            {errMsg}
        </div>
    {/if}
</div>

<style>
    input[type='date'] {
        padding-left: 0.2rem;
        width: 9rem;
    }

    .error {
        color: hsl(var(--error));
        margin-top: -0.4rem;
        padding-left: 0.1rem;
        font-size: 0.8rem;
    }

    label {
        font-size: 0.9em;
    }

    .label {
        width: 100%;
        margin-bottom: -0.37rem;
        padding: 0.1rem;
    }

    :global(.ui-shell) .admin-date-field {
        margin-bottom: 16px;
        width: var(--ui-date-width, 11.25rem);
        min-width: 0;
    }

    :global(.ui-shell) .admin-date-field input[type='date'] {
        width: 100%;
        height: var(--ui-field-height);
    }

    .date-control {
        position: relative;
    }

    .indicator {
        display: none;
    }

    @supports selector(:popover-open) {
        .date-control input[type='date'] {
            padding-inline-end: 0;
        }

        .indicator {
            display: grid;
            place-items: center;
            position: absolute;
            inset-block: 1px;
            inset-inline-end: 1px;
            width: 2.75rem;
            border-radius: var(--border-radius);
            background: var(--ui-surface, hsl(var(--bg)));
            --ui-button-height: calc(var(--ui-field-height, 2.75rem) - 4px);
            --ui-button-min-height: var(--ui-button-height);
            --ui-button-padding: 0;
        }

        .indicator > :global(div) > :global(button[popovertarget]) {
            width: 2.5rem;
            padding: 0;
        }

        /* Firefox keeps its native affordance beneath the opaque overlay. */
        input[type='date']::-webkit-calendar-picker-indicator {
            opacity: 0;
        }

        @media (forced-colors: active) {
            .indicator {
                background: Canvas;
            }
        }
    }

    .calendar {
        box-sizing: border-box;
        width: min(17rem, calc(100dvw - 18px));
        padding: 0.5rem;
    }

    .selectors {
        display: flex;
        justify-content: space-between;
        margin-bottom: 0.25rem;
    }

    .week {
        display: grid;
        grid-template-columns: repeat(7, minmax(0, 1fr));
        text-align: center;
        font-size: 0.8125rem;
    }

    .day {
        width: 100%;
        aspect-ratio: 1;
        padding: 0;
        border: 1px solid transparent;
        border-radius: var(--border-radius);
        background: transparent;
        color: var(--ui-text, hsl(var(--text)));
        font: inherit;
    }

    .day:not(:disabled) {
        cursor: pointer;
    }

    .day:disabled {
        opacity: 0.45;
    }

    .day:not(:disabled):hover {
        background: var(--ui-hover);
    }

    [aria-selected='true'] .day {
        background: var(--ui-tonal);
        color: var(--ui-primary);
        font-weight: 700;
    }

    .day[aria-current='date'] {
        border-color: var(--ui-outline);
    }

    .day:focus-visible {
        outline: 2px solid var(--ui-primary);
        outline-offset: -2px;
    }

    .visually-hidden {
        position: absolute;
        width: 1px;
        height: 1px;
        clip-path: inset(50%);
        overflow: hidden;
    }
</style>
