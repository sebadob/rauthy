<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import InputDate from '$lib5/form/InputDate.svelte';
    import IconStop from '$icons/IconStop.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import type { PropsInputDateCombo } from '$lib5/form/props.ts';
    import { fmtDateInput, fmtTimeInput } from '$utils/form';
    import InputTime from '$lib5/form/InputTime.svelte';

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

<div class="container">
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
                    ariaLabel={label}
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
</style>
