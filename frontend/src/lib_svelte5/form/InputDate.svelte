<script lang="ts">
    import {slide} from "svelte/transition";
    import {dayToString, fmtDateInput, getWeeksInMonth, type Day} from "$utils/form";
    import {untrack} from "svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {CalendarDate, getDayOfWeek, parseDate} from "@internationalized/date";
    import Popover from "$lib5/Popover.svelte";
    import Options from "$lib5/Options.svelte";
    import Button from "$lib5/button/Button.svelte";
    import IconCalendar from "$icons/IconCalendar.svelte";
    import type {PropsInputDate} from "$lib5/form/props.ts";

    let {
        id,
        name,
        value = $bindable(fmtDateInput()),
        label = '',
        errMsg = '',
        disabled,
        min = '1900-01-01',
        max = '2100-01-01',
        required,

        onEnter,
        onLeft,
        onRight,
        onUp,
        onDown,
    }: PropsInputDate = $props();

    $inspect(value).with(() => {
        if (value.includes('T')) {
            console.warn('invalid date format for InputDate, expected dateStr only', value);
        }
    });
    $inspect(min).with(() => {
        if (min.includes('T')) {
            console.warn('invalid min format for InputDate, expected dateStr only', min);
        }
    });
    $inspect(max).with(() => {
        if (max.includes('T')) {
            console.warn('invalid max format for InputDate, expected dateStr only', value);
        }
    });

    const today = parseDate(fmtDateInput());
    const todayStr = today.toString();

    let t = useI18n();
    let isErr = $state(false);
    let close: undefined | (() => void) = $state();

    let refInput: undefined | HTMLInputElement;
    let refMonth: undefined | HTMLButtonElement = $state();
    let refYear: undefined | HTMLButtonElement = $state();
    let refMonthContainer: undefined | HTMLDivElement = $state();

    let date = $derived(value ? parseDate(value) : today);
    let dayMin = $derived(parseDate(min));
    let dayMax = $derived(parseDate(max));

    let month = $state(untrack(() => t.common.months[date.month - 1]));
    let monthIdx = $derived(t.common.months.findIndex(m => m === month) + 1)
    let year = $state(untrack(() => date.year));
    let yearOptions = $state([untrack(() => year)]);
    let weeks = $derived(getWeeksInMonth(date, t.lang))

    $effect(() => {
        let yearMin = Number.parseInt(min.slice(0, 4));
        let yearMax = Number.parseInt(max.slice(0, 4));

        let opts = [];
        for (let i = yearMin; i <= yearMax; i++) {
            opts.push(i);
        }
        yearOptions = opts;
    });

    $effect(() => {
        if (value) {
            let dt = untrack(() => parseDate(value));
            dt = dt.set({year, month: monthIdx});
            value = dt.toString();
        }
    });

    $effect(() => {
        if (refMonthContainer && value) {
            requestAnimationFrame(() => {
                let days = refMonthContainer?.getElementsByTagName('time') || [];
                for (let day of days) {
                    if (day.getAttribute('datetime') === value) {
                        let button = day.parentElement?.parentElement;
                        button?.focus();
                        break;
                    }
                }
            })
        }
    });

    function isDataActive(day: Day) {
        let calDate = parseDate(dayToString(day));
        if (calDate.compare(dayMin) < 0 || calDate.compare(dayMax) > 0) {
            return false;
        }
        return monthIdx === day.month;
    }

    function setToFirstOfMonth() {
        value = new CalendarDate(year, monthIdx, 1).toString();
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        event.preventDefault();
        isErr = true;
    }

    function onClick(ev: Event, datetime: string) {
        ev.preventDefault();
        value = datetime;
        close?.();
    }

    function onKeyDownCalendar(direction: 'left' | 'right' | 'up' | 'down', datetime: string) {
        let before = parseDate(datetime);
        let after = parseDate(datetime);

        switch (direction) {
            case 'left':
                after = before.subtract({days: 1});
                break;
            case 'right':
                after = before.add({days: 1});
                break;
            case 'up':
                after = before.subtract({weeks: 1});
                break;
            case 'down':
                after = before.add({weeks: 1});
                break;
        }

        if (after.month !== before.month) {
            // if we have a change in month, we want to focus month / year selectors instead,
            // which is a way better UX than simply scrolling though each month
            let weekday = getDayOfWeek(after, t.lang);
            if (weekday < 4) {
                refMonth?.focus();
            } else {
                refYear?.focus();
            }
        } else {
            value = after.toString();
        }
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

    function onToggle(newState: string) {
        switch (newState) {
            case 'open':
                if (!value) {
                    value = todayStr;
                    requestAnimationFrame(() => reportValidity());
                }
                break;
            case 'closed':
                refMonthContainer = undefined;
                break;

        }
    }

    function reportValidity() {
        isErr = !refInput?.reportValidity();
    }

</script>

<div>
    <div class="flex">
        <input
                bind:this={refInput}
                type="date"
                {id}
                {name}
                title={label}
                aria-label={label}
                bind:value

                {disabled}
                aria-disabled={disabled}
                {required}
                aria-required={required || false}
                aria-invalid={isErr}
                {min}
                {max}
                step="any"

                {oninvalid}
                onkeydown={onKeyDownInput}
                onblur={() => reportValidity()}
        />

        <div class="relative">
            <div class="absolute patch"></div>
            <div class="absolute indicator">
                <Popover
                        ariaLabel="Show Popover Example"
                        offsetLeft="-7.5rem"
                        offsetTop="-.2rem"
                        bind:close
                        {onToggle}
                        btnInvisible
                        lazy
                >
                    {#snippet button()}
                        <div title={label} class="pointer">
                            <IconCalendar color="hsl(var(--text)" width="1.2rem"/>
                        </div>
                    {/snippet}
                    <div class="popup">
                        <div class="flex space-between">
                            <div>
                                <Options
                                        bind:ref={refMonth}
                                        ariaLabel={t.common.month}
                                        options={t.common.months}
                                        bind:value={month}
                                        maxHeight="13rem"
                                        borderless
                                        onRight={() => refYear?.focus()}
                                        onDown={setToFirstOfMonth}
                                />
                            </div>
                            <div>
                                <Options
                                        bind:ref={refYear}
                                        ariaLabel={t.common.year}
                                        options={yearOptions}
                                        bind:value={year}
                                        maxHeight="13rem"
                                        borderless
                                        onLeft={() => refMonth?.focus()}
                                        onDown={setToFirstOfMonth}
                                />
                            </div>
                        </div>

                        <div class="week">
                            {#each t.common.weekDaysShort as label}
                                <div class="day font-label weekLabel">
                                    {label}
                                </div>
                            {/each}
                        </div>

                        <div bind:this={refMonthContainer} class="month">
                            {#each weeks as week}
                                <div class="week">
                                    {#each week.days as day}
                                        {@const datetime = dayToString(day)}
                                        {@const dataActive = isDataActive(day)}
                                        <div class="day">
                                            <Button
                                                    onclick={ev => onClick(ev, datetime)}
                                                    isDisabled={!dataActive}
                                                    invisible
                                                    onLeft={() => onKeyDownCalendar('left', datetime)}
                                                    onRight={() => onKeyDownCalendar('right', datetime)}
                                                    onUp={() => onKeyDownCalendar('up', datetime)}
                                                    onDown={() => onKeyDownCalendar('down', datetime)}
                                            >
                                                <time
                                                        data-today={datetime === todayStr}
                                                        data-active={dataActive}
                                                        data-selected={datetime === value}
                                                        data-weekend={day.isWeekend}
                                                        {datetime}
                                                >
                                                    {day.day}
                                                </time>
                                            </Button>
                                        </div>
                                    {/each}
                                </div>
                            {/each}
                        </div>
                    </div>
                </Popover>
            </div>
        </div>
    </div>

    <div class="label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
        {#if isErr}
            <div class="error" transition:slide>
                {errMsg}
            </div>
        {/if}
    </div>
</div>

<style>
    input[type="date"] {
        padding-left: .2rem;
        width: 9rem;
    }

    .day {
        width: 2rem;
        border-radius: var(--border-radius);
        text-align: center;
        transition: all 150ms ease-in-out;
    }

    .day time {
        padding: .2rem .33rem;
        color: hsl(var(--text));
        font-weight: normal;
    }

    time[data-today="true"] {
        color: hsl(var(--accent));
    }

    /*.day:has(time[data-active="true"]:hover) {*/
    /*    background: hsl(var(--bg-high));*/
    /*}*/

    time[data-active="true"]:hover {
        color: hsl(var(--action));
        transform: scale(1.05);
    }

    time[data-weekend="true"] {
        color: hsl(var(--error));
    }

    time[data-selected="true"] {
        color: hsl(var(--action));
    }

    time[data-active="false"] {
        color: hsl(var(--bg-high));
    }

    .error {
        margin-top: -.5rem;
        color: hsl(var(--error));
    }

    .indicator {
        top: -.7rem;
        right: 0;
        padding: 2px 5px 0 5px;
    }

    label, .error {
        line-height: 1.1rem;
        font-size: .9rem;
    }

    label {
        color: hsla(var(--text) / .8);
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-top: -1.1rem;
        padding: .5rem;
    }

    .month {
        display: flex;
        flex-direction: column;
    }

    /* Just to hide the native browser picker indicator */
    .patch {
        width: 2rem;
        height: 2rem;
        top: calc(-1rem - 1px);
        right: 0;
        background: hsl(var(--bg));
    }

    .popup {
        padding: .5rem;
        border-radius: var(--border-radius);
        cursor: default;
    }

    .week {
        display: inline-flex;
    }

    .weekLabel {
        height: 1.2rem;
        font-size: .7rem;
        color: hsla(var(--text) / .66);
    }
</style>
