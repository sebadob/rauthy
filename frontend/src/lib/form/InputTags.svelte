<script lang="ts">
    import { genKey } from '$utils/helpers';
    import Button from '$lib5/button/Button.svelte';
    import IconStop from '$icons/IconStop.svelte';
    import { useI18n } from '$state/i18n.svelte';

    let {
        ref = $bindable(),
        typ = 'text',
        id = genKey(),
        name,
        values = $bindable([]),
        datalist,
        label = '',

        placeholder = '',
        disabled = false,

        maxLength,
        min,
        max,

        required = false,
        pattern,
        isError = $bindable(false),
        errMsg,

        width = 'inherit',
        maxHeightList = 'inherit',

        onBlur,
        onEnter,
        onInput,
        onSubmit,
    } = $props<{
        ref?: undefined | HTMLInputElement;
        typ?: 'text' | 'email' | 'url';
        id?: string;
        name?: string;
        values: string[];
        datalist?: string[];
        label?: string;
        placeholder?: string;
        errMsg?: string;
        disabled?: boolean | null | undefined;
        maxLength?: number | null | undefined;
        min?: string;
        max?: string;
        required?: boolean;
        pattern?: string;
        isError?: boolean;
        width?: string;
        maxHeightList?: string;

        onBlur?: () => void;
        onEnter?: () => void;
        onInput?: (value: string) => void;
        onSubmit?: (event: Event & { currentTarget: EventTarget & HTMLInputElement }) => void;
    }>();

    let t = useI18n();
    const idDatalist = genKey();

    let value = $state('');
    let list = $derived(datalist && datalist.length > 0 ? idDatalist : undefined);

    function deleteValue(value: string) {
        // for some reason TS complains about any type, when its string[]
        // @ts-ignore
        values = values.filter(v => v !== value);
    }

    function onblur(event: FocusEvent & { currentTarget: EventTarget & HTMLInputElement }) {
        isValid();
        onBlur?.();
        handleOnEnter();
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        isValid();
        onInput?.(value);
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        event.preventDefault();
        isError = true;
    }

    function onkeydown(ev: KeyboardEvent) {
        switch (ev.code) {
            case 'Enter':
                ev.preventDefault();
                handleOnEnter();
                onEnter?.();
                break;
        }
    }

    function handleOnEnter() {
        if (value && isValid()) {
            values.push(value);
            value = '';
        }
    }

    function focus() {
        ref?.focus();
        ref?.scrollIntoView({ behavior: 'smooth', block: 'center' });
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

<div role="none" class="container" onclick={focus} style:width>
    <ul style:max-height={maxHeightList}>
        {#each values as value}
            <li class="value">
                <div class="label">
                    {value}
                </div>
                <div class="btnClose">
                    <Button invisible onclick={() => deleteValue(value)}>
                        <IconStop width="1.2rem" />
                    </Button>
                </div>
            </li>
        {/each}
        <li>
            <input
                bind:this={ref}
                type={typ}
                {id}
                {name}
                {list}
                bind:value
                autocomplete="off"
                title={errMsg}
                aria-label={label || placeholder}
                {placeholder}
                aria-placeholder={placeholder}
                {disabled}
                aria-disabled={disabled}
                required={required && values.length < 1}
                aria-required={required && values.length < 1}
                maxlength={maxLength || undefined}
                min={min || undefined}
                max={max || undefined}
                {pattern}
                {onkeydown}
                {oninput}
                {oninvalid}
                {onblur}
            />

            {#if datalist && datalist.length > 1}
                <datalist id={idDatalist} class="absolute">
                    {#each datalist as value}
                        <option {value}></option>
                    {/each}
                </datalist>
            {/if}
        </li>
    </ul>
    <label for={id} class="font-label noselect" data-required={required}>
        {label}
        {#if isError}
            {#if isError}
                <span class="err">
                    {errMsg || t.common.invalidInput}
                </span>
            {/if}
        {/if}
    </label>
</div>

<style>
    datalist {
        display: block;
    }

    input {
        min-width: 3rem;
        margin: 0;
        padding: 0;
        background: transparent;
        border: none;
    }

    input:invalid {
        color: hsl(var(--error));
    }

    label {
        color: hsla(var(--text) / 0.8);
        margin-left: 0.25rem;
    }

    ul {
        padding: 0;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex-wrap: wrap;
        overflow-y: scroll;
    }

    li {
        margin: 0;
        padding: 0 0.25rem 0 0.35rem;
        display: flex;
        list-style: none;
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .container {
        margin: 1rem 0;
        max-width: 100%;
        padding: 0.25rem 0.25rem 0 0.25rem;
        background: hsla(var(--bg-high) / 0.2);
        border: 1px solid hsla(var(--bg-high));
        border-radius: var(--border-radius);
        cursor: text;
    }

    .btnClose {
        margin-top: 0.12rem;
        margin-bottom: -0.5rem;
    }

    .value {
        background: hsla(var(--accent) / 0.2);
        overflow: auto;
    }
</style>
