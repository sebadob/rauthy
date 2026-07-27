<script lang="ts">
    import { slide } from 'svelte/transition';
    import Options from '$lib5/Options.svelte';
    import Popover from '$lib5/Popover.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { untrack } from 'svelte';
    import { fmtTimeInput } from '$utils/form';
    import IconClock from '$icons/IconClock.svelte';

    let {
        id,
        name = 'time',
        value = $bindable(fmtTimeInput()),
        label = '',
        ariaLabel = '',
        errMsg = '',
        disabled,
        min = '00:00',
        max = '23:59',
        required,
        width,
        openTop = false,
        noEmptyLabelHeight = false,

        onEnter,
        onLeft,
        onRight,
        onUp,
        onDown,
    }: {
        id?: string;
        name?: string;
        value?: string;
        label?: string;
        ariaLabel?: string;
        errMsg?: string;
        disabled?: boolean;
        min?: string;
        max?: string;
        required?: boolean;
        width?: string;
        openTop?: boolean;
        noEmptyLabelHeight?: boolean;

        onEnter?: () => void;
        onLeft?: () => void;
        onRight?: () => void;
        onUp?: () => void;
        onDown?: () => void;
    } = $props();

    $inspect(min, max).with(() => {
        if (
            min.length !== 5 ||
            min.charAt(2) !== ':' ||
            max.length !== 5 ||
            max.charAt(2) !== ':'
        ) {
            console.error(
                'min and max values for InputTime must be in the format of 00:00 - 23:59',
            );
        }
    });

    let t = useI18n();
    let isErr = $state(false);
    let close: undefined | (() => void) = $state();

    let refInput: undefined | HTMLInputElement;
    let refHours: undefined | HTMLButtonElement = $state();
    let refMinutes: undefined | HTMLButtonElement = $state();

    let hourMin = $derived(Number.parseInt(min.slice(0, 2)));
    let hourMax = $derived(Number.parseInt(max.slice(0, 2)));
    let minuteMin = $derived(Number.parseInt(min.slice(3, 5)));
    let minuteMax = $derived(Number.parseInt(max.slice(3, 5)));

    let hour = $state(untrack(() => value.slice(0, 2)));
    let minute = $state(untrack(() => value.slice(3, 5)));
    let isPopover = $state(false);

    let optionHour = $derived.by(() => {
        let opts = [];
        for (let i = hourMin; i <= hourMax; i++) {
            if (i < 10) {
                opts.push('0' + i);
            } else {
                opts.push('' + i);
            }
        }
        return opts;
    });
    let optionMinute = $derived.by(() => {
        let opts = [];
        for (let i = minuteMin; i <= minuteMax; i++) {
            if (i < 10) {
                opts.push('0' + i);
            } else {
                opts.push('' + i);
            }
        }
        return opts;
    });

    $effect(() => {
        if (value.length === 5) {
            hour = value.slice(0, 2);
            minute = value.slice(3, 5);
        }
    });

    $effect(() => {
        if (isPopover) {
            value = `${hour}:${minute}`;
        }
    });

    $effect(() => {
        if (minute) {
            refMinutes?.focus();
        }
    });

    $effect(() => {
        if (hour) {
            refHours?.focus();
        }
    });

    function oninput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        let val = event.currentTarget.value;

        if (val.length === 1 && val.charAt(0) === ':') {
            val = '00:';
        }

        if (val.length === 2) {
            if (val.charAt(1) === ':') {
                val = `0${val}:`;
            } else {
                val += ':';
            }
        }

        val = val.replace(/\D/g, '');
        if (val.length >= 3) {
            val = `${val.slice(0, 2)}:${val.slice(2)}`;
        }

        value = val;
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        event.preventDefault();
        isErr = true;
    }

    function onkeydown(ev: KeyboardEvent) {
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
                // set now() as default in case of invalid input
                if (value.length !== 5) {
                    value = fmtTimeInput();
                }

                isPopover = true;
                refHours?.focus();
                if (!value) {
                    requestAnimationFrame(() => reportValidity());
                }
                break;
            default:
                isPopover = false;
                break;
        }
    }

    function reportValidity() {
        let intHour = Number.parseInt(hour);
        let intMinute = Number.parseInt(minute);

        if (
            intHour < hourMin ||
            intHour > hourMax ||
            intMinute < minuteMin ||
            intMinute > minuteMax
        ) {
            refInput?.setCustomValidity(errMsg);
        }

        isErr = !refInput?.reportValidity();
    }
</script>

<div style:width>
    <div
        aria-live="assertive"
        class="label"
        style:height={noEmptyLabelHeight ? 'inherit' : '1.75rem'}
    >
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
    </div>

    <div class="flex">
        <input
            bind:this={refInput}
            type="text"
            {id}
            {name}
            title={errMsg}
            aria-label={ariaLabel || label}
            bind:value
            {disabled}
            aria-disabled={disabled}
            {required}
            aria-required={required || false}
            aria-invalid={isErr}
            minlength="5"
            maxlength="5"
            {oninput}
            {oninvalid}
            {onkeydown}
            onfocus={() => refInput?.select()}
            onblur={() => reportValidity()}
        />

        <div class="relative">
            <div class="absolute indicator">
                <Popover
                    ariaLabel="Show Popover Example"
                    offsetLeft="-3.8rem"
                    offsetTop={openTop ? '-5rem' : '-.1rem'}
                    bind:close
                    {onToggle}
                    btnInvisible
                >
                    {#snippet button()}
                        <div title={label} class="pointer">
                            <IconClock color="hsl(var(--text)" width="1.2rem" />
                        </div>
                    {/snippet}
                    <div class="popup">
                        <div class="flex space-between">
                            <div>
                                <Options
                                    bind:ref={refHours}
                                    ariaLabel={t.common.hours}
                                    options={optionHour}
                                    bind:value={hour}
                                    maxHeight="13rem"
                                    borderless
                                    onRight={() => refMinutes?.focus()}
                                />
                            </div>
                            <div class="colon">:</div>
                            <div>
                                <Options
                                    bind:ref={refMinutes}
                                    ariaLabel={t.common.minutes}
                                    options={optionMinute}
                                    bind:value={minute}
                                    maxHeight="13rem"
                                    borderless
                                    onLeft={() => refHours?.focus()}
                                />
                            </div>
                        </div>
                    </div>
                </Popover>
            </div>
        </div>
    </div>

    {#if isErr}
        <div class="error" transition:slide={{ duration: 150 }}>
            {errMsg}
        </div>
    {/if}
</div>

<style>
    input {
        padding-left: 0.3rem;
        width: 5.25rem;
        letter-spacing: 0.05rem;
        user-select: all;
    }

    label,
    .error {
        font-size: 0.9rem;
    }

    .label {
        /*height: 1.75rem;*/
        width: 100%;
        margin-bottom: -0.37rem;
        padding: 0.1rem;
        flex-wrap: wrap;
    }

    .colon {
        margin: 0 0.25rem;
    }

    .error {
        margin-top: -0.5rem;
        color: hsl(var(--error));
    }

    .indicator {
        top: -0.7rem;
        right: 0;
        padding: 2px 5px 0 5px;
    }

    .popup {
        padding: 0.5rem;
        border-radius: var(--border-radius);
        cursor: default;
    }
</style>
