<script lang="ts">
    import {genKey} from "$utils/helpers";
    import Button from "$lib5/button/Button.svelte";
    import IconMagnify from "$icons/IconMagnify.svelte";
    import Options from "$lib5/Options.svelte";
    import IconBackspace from "$icons/IconBackspace.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import {useI18n} from "$state/i18n.svelte";

    let t = useI18n();
    let ta = useI18nAdmin();

    let {
        id = genKey(),
        ariaLabel,
        value = $bindable(''),
        placeholder = t.common.search,
        datalist,
        options,
        option = $bindable(),
        pattern,
        focus = $bindable(),
        width = '100%',
        borderless,
        onSearch,
        onTab,
        onUp,
        onDown,
        onFocus,
    }: {
        id?: string;
        ariaLabel?: string;
        value?: string;
        placeholder?: string;
        datalist?: string[];
        options?: string[];
        option?: string;
        pattern?: string;
        focus?: () => void,
        width?: string;
        borderless?: boolean;
        onSearch?: (value: string, option?: string) => void,
        onTab?: (value: string) => void,
        onUp?: (value: string) => void,
        onDown?: (value: string) => void,
        onFocus?: () => void,
    } = $props();

    const idDatalist = genKey();

    let ref: undefined | HTMLElement = $state();
    let list = $derived(datalist && datalist.length > 0 ? idDatalist : undefined);

    $effect(() => {
        focus = doFocus;
    });

    function onkeydown(ev: KeyboardEvent) {
        switch (ev.code) {
            case 'Enter':
                search();
                break;
            case 'Tab':
                onTab?.(value);
                break;
            case 'ArrowUp':
                onUp?.(value);
                break;
            case 'ArrowDown':
                onDown?.(value);
                break;
        }
    }

    function search() {
        onSearch?.(value, option);
    }

    function doFocus() {
        ref?.focus();
    }
</script>

<search
        class="flex container"
        style:border={borderless ? undefined : '1px solid hsl(var(--bg-high))'}
        style:width
>
    {#if options}
        <div class="options">
            <Options
                    ariaLabel={ta.common.searchOptions}
                    {options}
                    bind:value={option}
                    borderless
                    offsetLeft="-.25rem"
            />
        </div>
    {/if}

    <input
            bind:this={ref}
            type="search"
            {id}
            {list}
            autocomplete="off"
            aria-label={ariaLabel || t.common.search}
            {pattern}
            {placeholder}
            onclick={ev => ev.stopPropagation()}
            {onkeydown}
            onfocus={() => onFocus?.()}
            bind:value
    />

    {#if datalist}
        <datalist id={idDatalist} class="absolute">
            {#each datalist as value}
                <option {value}></option>
            {/each}
        </datalist>
    {/if}

    <div class="relative">
        <div class="absolute btnDelete">
            <Button ariaLabel={t.common.delete} invisible onclick={() => value = ''}>
                <span class="backspace">
                    <IconBackspace color="hsla(var(--text) / .5)"/>
                </span>
            </Button>
        </div>
    </div>

    {#if onSearch}
        <div class="btnSearch">
            <Button ariaLabel={t.common.search} invisible onclick={search}>
                <IconMagnify/>
            </Button>
        </div>
    {/if}
</search>

<style>
    input {
        padding-right: 1.9rem;
        border: none;
        margin: 0;
    }

    datalist {
        display: block;
    }

    .backspace {
        position: relative;
        bottom: .2rem;
    }

    .btnDelete {
        top: -.55rem;
        right: .3rem;
        opacity: .8;
    }

    .btnSearch {
        margin: 2px 2px 2px 0;
        padding: 0 .25rem;
        background: hsl(var(--bg-high));
        border-left: 1px solid hsl(var(--bg-high));
        border-radius: 0 2px 2px 0;
    }

    .container {
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .options {
        padding: 0 .25rem;
        background: hsl(var(--bg-high));
        border-right: 1px solid hsl(var(--bg-high));
        border-radius: 2px 0 0 2px;
    }
</style>
