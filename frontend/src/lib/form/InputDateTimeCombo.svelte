<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import InputDate from '#lib/form/InputDate.svelte';
    import IconStop from '#icons/IconStop.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { PropsInputDateCombo } from '#lib/form/props.ts';
    import { fmtDateInput, fmtTimeInput } from '#utils/form.ts';
    import InputTime from '#lib/form/InputTime.svelte';

    let {
        id,
        name,
        value = $bindable(fmtDateInput()),
        label = '',
        labelTime = '',
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

        timeName,
        timeValue = $bindable(fmtTimeInput()),
        timeErrMsg,
        timeMin,
        timeMax,

        withDelete,
        withTime,
        openTop = false,
    }: PropsInputDateCombo = $props();

    let t = useI18n();

    function reset() {
        value = '';
        timeValue = '--:--';
    }
</script>

<div class="container" class:withTime>
    <div class="inputs-wrapper">
        <div class="inputs">
            <InputDate
                {id}
                {name}
                bind:value
                {label}
                {errMsg}
                {disabled}
                {min}
                {max}
                {required}
                {onEnter}
                {onLeft}
                {onRight}
                {onUp}
                {onDown}
                {openTop}
            />
            {#if withTime}
                <InputTime
                    name={timeName}
                    ariaLabel={labelTime || label}
                    label={labelTime}
                    bind:value={timeValue}
                    errMsg={timeErrMsg}
                    min={timeMin}
                    max={timeMax}
                    {disabled}
                    {required}
                    {openTop}
                    noEmptyLabelHeight={!label && !labelTime}
                />
            {/if}
        </div>
        {#if withDelete}
            <div class="delete">
                <Button ariaLabel={t.common.delete} invisible onclick={reset}>
                    <span title={t.common.delete}>
                        <IconStop color="hsla(var(--error) / .8)" width="1.2rem" />
                    </span>
                </Button>
            </div>
        {/if}
    </div>
</div>

<style>
    .inputs-wrapper {
        display: flex;
        gap: 0.25rem;
    }

    .delete {
        display: flex;
        align-items: end;
        padding-bottom: 0.45rem;
    }

    .inputs {
        display: flex;
    }

    :global(.ui-shell) .inputs-wrapper {
        align-items: start;
        gap: 0.5rem;
    }
    :global(.ui-shell) .delete {
        padding: calc(var(--ui-label-height) + var(--ui-label-gap)) 0 0;
        --ui-button-height: var(--ui-field-height);
        --ui-button-min-height: var(--ui-field-height);
    }
    :global(.ui-shell) .inputs {
        display: grid;
        flex: 1;
        min-width: 0;
        grid-template-columns: minmax(0, 1fr);
        --ui-date-width: 100%;
    }
    :global(.ui-shell) .withTime .inputs {
        grid-template-columns: minmax(0, 1fr) minmax(5.5rem, 0.65fr);
        gap: 0.5rem;
        --ui-time-width: 100%;
    }
    :global(.ui-shell) .inputs > :global(.admin-date-field) {
        margin: 0;
    }
</style>
