<script lang="ts">
    import type {FullAutoFill} from "svelte/elements";
    import IconClipboard from "$icons/IconClipboard.svelte";
    import IconEyeSlash from "$icons/IconEyeSlash.svelte";
    import IconEye from "$icons/IconEye.svelte";
    import {slide} from "svelte/transition";
    import {useI18n} from "$state/i18n.svelte.ts";

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
        maxLength = 128,
        required = true,
        pattern,
        errMsg,
        width = 'inherit',
        showCopy = false,

        onBlur,
        onEnter,
        onInput,
    }: {
        ref?: undefined | HTMLInputElement,
        type?: string,
        id?: undefined | string,
        name?: string,
        value?: string,
        label?: string,
        ariaLabel?: string,
        autocomplete?: FullAutoFill | null | undefined,
        placeholder: string,
        disabled?: boolean | null | undefined,
        maxLength?: number | null | undefined,
        min?: string,
        max?: string,
        required?: boolean,
        pattern?: string,
        errMsg?: string,
        width?: string,
        showCopy?: boolean,

        onBlur?: () => void,
        onEnter?: () => void,
        onInput?: () => void,
    } = $props();

    let t = useI18n();

    let isError = $state(false);

    function copy() {
        if (navigator.clipboard) {
            navigator.clipboard.writeText(value);
        } else {
            console.error("Copy to clipboard is only available in secure contexts");
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
        console.log('on blur', label);
        isValid();
        onBlur?.()
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        console.log('on input', label);
        isValid();
        onInput?.();
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        console.log('on invalid', label);
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

<div style:width={width}>
    <div class="input-row">
        <input
                bind:this={ref}
                {type}
                {id}
                {name}
                aria-label={ariaLabel}
                style:padding-right={showCopy ? '3.8rem' : '2.2rem'}
                bind:value

                {autocomplete}
                {placeholder}
                {disabled}

                required={required || undefined}
                maxlength={maxLength || undefined}
                pattern={pattern || undefined}

                {oninput}
                {oninvalid}
                {onblur}
                {onkeydown}
        />

        <div class="rel">
            {#if showCopy}
                <div
                        role="button"
                        tabindex="0"
                        class="btn clip"
                        onclick={copy}
                        onkeydown={copy}
                >
                    <IconClipboard/>
                </div>
            {/if}

            <div
                    role="button"
                    tabindex="0"
                    class="btn show"
                    onclick={toggleView}
                    onkeydown={toggleView}
            >
                {#if type === 'password'}
                    <IconEyeSlash/>
                {:else}
                    <IconEye/>
                {/if}
            </div>
        </div>
    </div>
</div>

<div class="label">
    <label for={id} class="font-label noselect" data-required={required}>
        {label}
    </label>
    {#if isError}
        <div class="error" transition:slide>
            {#if !label}
                <div class="nolabel"></div>
            {/if}
            {errMsg || t.common.invalidInput}
        </div>
    {/if}
</div>

<style>
    .error {
        margin-top: -.5rem;
        color: hsl(var(--error));
    }

    .input-row {
        display: flex;
        flex-direction: row;
    }

    label, .error {
        line-height: 1.1rem;
        font-size: .9rem;
    }

    label {
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-top: -1.1rem;
        padding: .5rem;
    }

    .nolabel {
        height: .8rem;
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
