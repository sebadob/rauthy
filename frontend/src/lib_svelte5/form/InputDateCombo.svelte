<script lang="ts">
    import Button from "$lib5/Button.svelte";
    import InputDate from "$lib5/form/InputDate.svelte";
    import IconStop from "$icons/IconStop.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import type {PropsInputDateCombo} from "$lib5/form/props.ts";
    import {genKey} from "$utils/helpers.ts";
    import {fmtDateInput} from "$utils/form.ts";

    let {
        id = genKey(),
        name,
        value = $bindable(fmtDateInput()),
        label = '',
        title = '',
        disabled,
        min = '1900-01-01',
        max = '2100-01-01',
        required,
        onEnter,
        onLeft,
        onRight,
        onUp,
        onDown,

        withDelete,
    }: PropsInputDateCombo = $props()

    let t = useI18n();
</script>

<div class="container">
    <InputDate
            {id}
            {name}
            bind:value
            {label}
            {title}
            {disabled}
            {min}
            {max}
            {required}
            {onEnter}
            {onLeft}
            {onRight}
            {onUp}
            {onDown}
    />
    {#if withDelete}
        <div class="delete">
            <Button
                    ariaLabel={t.common.delete}
                    invisible
                    onclick={() => value = ''}
            >
                <span title={t.common.delete}>
                    <IconStop color="hsla(var(--error) / .8)" width="1.2rem"/>
                </span>
            </Button>
        </div>
    {/if}
</div>

<style>

    .container {
        display: flex;
        gap: .25rem;
    }

    .delete {
        margin-top: .85em;
    }
</style>