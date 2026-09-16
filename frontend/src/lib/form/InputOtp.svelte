<script lang="ts">
    import { slide } from 'svelte/transition';
    import { useI18n } from '$state/i18n.svelte';
    import type { FullAutoFill } from 'svelte/elements';
    import { PATTERN_OTP_CODE } from '$utils/patterns';
    import Template from '$lib5/Template.svelte';
    import { TPL_OTP_LENGTH } from '$utils/constants';

    let t = useI18n();
    let otpSize = $state(8);

    let {
        ref = $bindable(),
        typ = 'text',
        id,
        name = 'otp',
        value = $bindable(''),
        label = t.mfa.otp.code,

        autocomplete = 'one-time-code',
        errMsg,
        isError = $bindable(false),

        onBlur,
        onEnter,
        onInput,
    }: {
        ref?: undefined | HTMLInputElement;
        typ?: string;
        id?: string;
        name?: string;
        value?: string | number | null;
        label?: string;
        autocomplete?: FullAutoFill | null | undefined;
        placeholder?: string;
        errMsg?: string;
        isError?: boolean;

        onBlur?: () => void;
        onEnter?: () => void;
        onInput?: () => void;
    } = $props();

    let placeholder = $derived('0'.repeat(otpSize));

    function onblur(event: FocusEvent & { currentTarget: EventTarget & HTMLInputElement }) {
        isValid();
        onBlur?.();
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        isValid();
        onInput?.();
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        event.preventDefault();
        isError = true;
    }

    function onkeydown(ev: KeyboardEvent) {
        if (ev.code === 'Enter') {
            if (isValid()) {
                onEnter?.();
            }
        }
    }

    function isValid() {
        let validity = ref?.validity;
        if (validity) {
            isError = !validity.valid;
            return validity.valid;
        }
        isError = false;
        return true;
    }
</script>

<Template id={TPL_OTP_LENGTH} bind:value={otpSize} />

<div>
    <div aria-live="assertive" class="label">
        <label for={id} class="font-label noselect" data-required="true">
            {label}
        </label>
    </div>
    <input
        bind:this={ref}
        type={typ}
        {id}
        {name}
        title={errMsg}
        aria-label={label || placeholder}
        bind:value
        {autocomplete}
        {placeholder}
        aria-placeholder={placeholder}
        minlength="6"
        maxlength="8"
        required={true}
        aria-required={true}
        aria-invalid={isError}
        pattern={PATTERN_OTP_CODE}
        class:invalid={isError}
        {oninput}
        {oninvalid}
        {onblur}
    />
    {#if isError}
        <div
            aria-relevant="all"
            class="error"
            class:errWithLabel={!!label}
            transition:slide={{ duration: 150 }}
        >
            {errMsg || t.common.invalidInput}
        </div>
    {/if}
</div>

<style>
    input {
        max-inline-size: 40rem;
        font-size: 2.5em;
        text-align: center;
        letter-spacing: 0.7em;
    }

    label {
        font-size: 0.9em;
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-bottom: -0.3rem;
        padding-left: 0.1rem;
        padding-top: 0.1rem;
    }

    .error {
        color: hsl(var(--error));
        margin-top: -0.4rem;
        padding-left: 0.1rem;
        font-size: 0.8rem;
    }
</style>
