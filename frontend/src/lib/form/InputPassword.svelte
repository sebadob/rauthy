<script lang="ts">
    import type { FullAutoFill } from 'svelte/elements';
    import IconClipboard from '$icons/IconClipboard.svelte';
    import IconEyeSlash from '$icons/IconEyeSlash.svelte';
    import IconEye from '$icons/IconEye.svelte';
    import { slide } from 'svelte/transition';
    import { useI18n } from '$state/i18n.svelte';
    import Button from '$lib5/button/Button.svelte';

    let {
        ref = $bindable(),
        type = 'password',
        id,
        name = 'password',
        value = $bindable(''),
        label = 'Password',
        ariaLabel = 'Password',
        autocomplete = 'current-password',
        placeholder = 'Password',
        disabled = false,
        maxLength,
        required,
        pattern,
        errMsg,
        width = 'inherit',
        showCopy = false,

        reportValidity = $bindable(),
        onBlur,
        onEnter,
        onInput,
    }: {
        ref?: undefined | HTMLInputElement;
        type?: string;
        id?: undefined | string;
        name?: string;
        value?: string;
        label?: string;
        ariaLabel?: string;
        autocomplete?: FullAutoFill | null | undefined;
        placeholder: string;
        disabled?: boolean | null | undefined;
        maxLength?: number | null | undefined;
        min?: string;
        max?: string;
        required?: boolean;
        pattern?: string;
        errMsg?: string;
        width?: string;
        showCopy?: boolean;

        reportValidity?: () => void;
        onBlur?: () => void;
        onEnter?: () => void;
        onInput?: () => void;
    } = $props();

    let t = useI18n();

    let isError = $state(false);

    reportValidity = isValid;

    function copy() {
        if (navigator.clipboard) {
            navigator.clipboard.writeText(value);
        } else {
            console.error('Copy to clipboard is only available in secure contexts');
        }
    }

    function toggleView() {
        if (type === 'password') {
            type = 'text';
        } else {
            type = 'password';
        }
    }

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

<div class="container" style:width>
    <div aria-live="assertive" class="label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
    </div>
    <div class="input-row">
        <input
            bind:this={ref}
            {type}
            {id}
            {name}
            title={errMsg}
            aria-label={ariaLabel}
            style:padding-right={showCopy ? '3.8rem' : '2.2rem'}
            bind:value
            {autocomplete}
            {placeholder}
            {disabled}
            required={required || undefined}
            aria-required={required || false}
            maxlength={maxLength || undefined}
            pattern={pattern || undefined}
            class:invalid={isError}
            {oninput}
            {oninvalid}
            {onblur}
            {onkeydown}
        />

        <div class="rel">
            {#if showCopy}
                <div class="btn clip">
                    <Button ariaLabel={t.common.copyToClip} invisible onclick={copy}>
                        <div title={t.common.copyToClip}>
                            <IconClipboard />
                        </div>
                    </Button>
                </div>
            {/if}

            <div class="btn show">
                {#if type === 'password'}
                    <Button ariaLabel={t.common.show} invisible onclick={toggleView}>
                        <div title={t.common.show}>
                            <IconEyeSlash />
                        </div>
                    </Button>
                {:else}
                    <Button ariaLabel={t.common.hide} invisible onclick={toggleView}>
                        <div title={t.common.hide}>
                            <IconEye />
                        </div>
                    </Button>
                {/if}
            </div>
        </div>
    </div>
    {#if isError}
        <div class="error" transition:slide={{ duration: 150 }}>
            {#if !label}
                <div class="nolabel"></div>
            {/if}
            {errMsg || t.common.invalidInput}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0.5rem 0;
    }

    .input-row {
        display: flex;
        flex-direction: row;
    }

    label {
        font-size: 0.9em;
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-bottom: -0.35rem;
        padding-left: 0.1rem;
        /*padding-top: 0.6rem;*/
    }

    .error {
        color: hsl(var(--error));
        margin-top: -0.4rem;
        padding-left: 0.1rem;
        font-size: 0.8rem;
    }

    .nolabel {
        height: 0.8rem;
    }

    .btn {
        position: absolute;
        top: 10px;
        right: 5px;
        margin-left: 100px;
        opacity: 0.85;
        cursor: pointer;
    }

    .clip {
        right: 32px;
        opacity: 0.85;
    }

    .show {
        opacity: 0.85;
    }

    .rel {
        position: relative;
        /*border: 1px solid green;*/
    }
</style>
