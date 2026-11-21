<script lang="ts">
    import { tick } from 'svelte';
    import { genKey } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import IconStop from '#icons/IconStop.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';

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
        validate,

        width = 'inherit',
        maxHeightList = 'inherit',

        onBlur,
        onEnter,
        onInput,
        onSubmit,
    }: {
        ref?: undefined | HTMLInputElement;
        typ?: 'text' | 'email' | 'url';
        id?: string;
        name?: string;
        values: string[];
        datalist?: string[];
        label?: string;
        placeholder?: string;
        errMsg?: string;
        // custom validation of the current input, returns an error message if it is invalid
        validate?: (value: string) => string | undefined;
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
    } = $props();

    let t = useI18n();
    const idDatalist = genKey();

    let value = $state('');
    let editing = $state<{ index: number; original: string }>();
    let validateErr = $state('');
    let list = $derived(datalist && datalist.length > 0 ? idDatalist : undefined);

    function deleteValue(value: string) {
        // for some reason TS complains about any type, when its string[]
        // @ts-ignore
        values = values.filter(v => v !== value);
    }

    async function startEdit(event: MouseEvent, index: number) {
        event.stopPropagation();
        if (!commitValue()) {
            ref?.focus();
            return;
        }
        editing = { index, original: values[index] };
        value = values[index];
        await tick();
        ref?.focus();
        ref?.select();
    }

    function resetEntry() {
        editing = undefined;
        value = '';
        validateErr = '';
        ref?.setCustomValidity('');
        isError = false;
    }

    function onblur() {
        commitValue();
        onBlur?.();
    }

    function oninput() {
        isValid();
        onInput?.(value);
    }

    function oninvalid(event: Event) {
        event.preventDefault();
        isError = true;
    }

    async function onkeydown(event: KeyboardEvent) {
        if (event.isComposing) return;
        if (event.key === 'Escape' && editing) {
            event.preventDefault();
            event.stopPropagation();
            resetEntry();
        } else if (event.key === 'Enter') {
            event.preventDefault();
            const wasEditing = !!editing;
            commitValue();
            onEnter?.();
            if (!wasEditing || editing) return;
        } else {
            return;
        }
        await tick();
        ref?.focus();
    }

    function commitValue() {
        if (!editing && !value) return true;
        if (!isValid()) return false;
        if (editing) {
            const { index, original } = editing;
            if (values[index] === original) {
                values[index] = value;
            }
            resetEntry();
        } else {
            values.push(value);
            value = '';
        }
        return true;
    }

    function focus() {
        ref?.focus();
        ref?.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }

    function isValid() {
        if (validate) {
            validateErr = (value && validate(value)) || '';
            ref?.setCustomValidity(validateErr);
        }

        let validity = ref?.validity;
        if (validity) {
            isError = !validity.valid;
            return validity.valid;
        }
        isError = false;
        return true;
    }
</script>

{#snippet entryField()}
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
        required={!!editing || (required && values.length < 1)}
        aria-required={!!editing || (required && values.length < 1)}
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
{/snippet}

<div role="none" class="container admin-tags-field" onclick={focus} style:width>
    <ul style:max-height={maxHeightList}>
        {#each values as tag, index}
            <li class="value" class:editing={editing?.index === index}>
                {#if editing?.index === index}
                    {@render entryField()}
                {:else}
                    <button
                        type="button"
                        class="label"
                        aria-label={`${t.common.edit}: ${tag}`}
                        {disabled}
                        onclick={event => startEdit(event, index)}>{tag}</button
                    >
                    <div class="btnClose">
                        <Button
                            ariaLabel={`${t.common.delete}: ${tag}`}
                            invisible
                            isDisabled={disabled || !!editing}
                            onclick={() => deleteValue(tag)}
                        >
                            <IconStop width="1.2rem" />
                        </Button>
                    </div>
                {/if}
            </li>
        {/each}
        {#if !editing}
            <li>{@render entryField()}</li>
        {/if}
    </ul>
    <label for={id} class="font-label noselect" data-required={required}>
        {label}
        {#if isError}
            {#if isError}
                <span class="err">
                    {validateErr || errMsg || t.common.invalidInput}
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
        overflow: auto;
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

    .value > .label {
        padding: 0;
        border: 0;
        background: transparent;
        color: inherit;
        font: inherit;
        text-align: left;
        overflow-wrap: anywhere;
        cursor: pointer;
    }

    .value > .label:hover:not(:disabled),
    .value > .label:focus-visible {
        text-decoration: underline;
    }

    .value.editing {
        flex: 1;
        min-width: min(14rem, 100%);
    }

    .value.editing input {
        width: 100%;
    }

    .value {
        background: hsla(var(--accent) / 0.2);
    }

    :global(.admin-shell) .admin-tags-field {
        display: flex;
        flex-direction: column;
        padding: 0;
        border: 0;
        background: transparent;
        margin: 16px 0;
    }
    :global(.admin-shell) .admin-tags-field > :global(label) {
        order: -1;
        margin: 0 0 6px;
    }
    :global(.admin-shell) .admin-tags-field > :global(ul) {
        min-height: 44px;
        padding: 6px;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        gap: 6px;
        overflow-y: auto;
    }
    :global(.admin-shell) .admin-tags-field :global(li) {
        padding: 0;
        align-items: center;
    }
    :global(.admin-shell) .admin-tags-field :global(li.value) {
        padding-left: 10px;
        background: var(--ui-tonal);
        border-radius: var(--border-radius);
        font-size: 12px;
    }
    :global(.admin-shell) .admin-tags-field :global(li) :global(.label) {
        margin: 0;
    }
    :global(.admin-shell) .admin-tags-field :global(.btnClose) {
        margin: 0;
    }
    :global(.admin-shell) .admin-tags-field :global(input) {
        min-height: 30px;
        padding: 3px 8px;
        border: 0 !important;
        outline: 0;
        background: transparent;
    }
</style>
