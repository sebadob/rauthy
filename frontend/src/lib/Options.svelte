<script lang="ts">
    import Popover from '$lib5/Popover.svelte';
    import Button from '$lib5/button/Button.svelte';
    import SearchBar from '$lib5/search_bar/SearchBar.svelte';
    import IconChevronDown from '$icons/IconChevronDown.svelte';
    import { untrack } from 'svelte';

    let {
        ref = $bindable(),
        ariaLabel,
        options = [],
        name,
        value = $bindable(),
        maxHeight = '16rem',
        offsetTop,
        offsetLeft,
        borderless = false,
        withSearch = false,
        onChange,
        onLeft,
        onRight,
        onUp,
        onDown,
    }: {
        ref?: undefined | HTMLButtonElement;
        ariaLabel: string;
        options: string[] | number[];
        name?: string;
        value?: string | number;
        maxHeight?: string;
        offsetTop?: string;
        offsetLeft?: string;
        borderless?: boolean;
        withSearch?: boolean;
        onChange?: (value: string | number | undefined) => void;
        onLeft?: () => void;
        onRight?: () => void;
        onUp?: () => void;
        onDown?: () => void;
    } = $props();

    $inspect(options).with(() => {
        if (options.length > 0 && typeof options[0] !== typeof value) {
            console.warn("type of 'options' does not match the one of 'value'", options, value);
        }
    });

    let refOptions: undefined | HTMLElement = $state();
    let close: undefined | (() => void) = $state();

    let selected = $state(untrack(() => withSearch) ? -1 : 0);
    let focusSearch: undefined | (() => void) = $state();

    let searchValue = $state('');
    let optionsFiltered = $derived.by(() => {
        if (!withSearch) {
            return options;
        }

        if (typeof value === 'string') {
            return options.filter(opt =>
                (opt as string).toLowerCase().includes(searchValue.toLowerCase()),
            );
        }

        let searchInt = Number.parseInt(searchValue) || value;
        return options.filter(opt => opt === searchInt);
    });

    let isInitialized = false;
    $effect(() => {
        let v = value;
        if (isInitialized) {
            onChange?.(v);
        } else {
            isInitialized = true;
        }
    });

    $effect(() => {
        if (selected === -1) {
            refOptions?.scrollTo({
                top: 0,
                behavior: 'smooth',
            });
        }

        if (withSearch) {
            if (selected < 0 || selected > optionsFiltered.length - 1) {
                selected = -1;
                focusSearch?.();
                return;
            }
        } else {
            if (selected < 0) {
                selected = optionsFiltered.length - 1;
            } else if (selected > optionsFiltered.length - 1) {
                selected = 0;
            }
            focusCurrElem();
        }
    });

    function focusCurrElem() {
        if (refOptions) {
            let button = refOptions.getElementsByTagName('button')[selected];
            button?.scrollIntoView({ behavior: 'smooth', block: 'center' });
            button?.focus();
        } else {
            console.error('refOptions is undefined');
        }
    }

    function onToggle(newState: string) {
        if (newState === 'open') {
            if (withSearch) {
                selected = -1;
                focusSearch?.();
            } else {
                selected = options.findIndex(opt => opt === value) || 0;
                focusCurrElem();
            }
        }
    }

    function onkeydown(ev: KeyboardEvent) {
        let code = ev.code;

        if (code === 'ArrowDown' || code === 'Tab') {
            ev.preventDefault();
            if (hasFilteredItems()) {
                selected += 1;
            }
        } else if (code === 'ArrowUp') {
            ev.preventDefault();
            if (hasFilteredItems()) {
                selected -= 1;
            }
        } else if (code === 'Enter') {
            if (selected > -1) {
                select(ev, optionsFiltered[selected]);
            } else if (selected === -1 && optionsFiltered.length === 1) {
                select(ev, optionsFiltered[0]);
            }
        }
    }

    function hasFilteredItems() {
        if (optionsFiltered.length > 0) {
            return true;
        }
        selected = -1;
        return false;
    }

    function select(ev: Event, option: string | number) {
        ev.preventDefault();
        value = option;
        searchValue = '';

        setTimeout(() => {
            close?.();
        }, 20);
    }
</script>

<Popover
    bind:ref
    ariaLabel={ariaLabel || value?.toString() || 'Options'}
    roleButton="combobox"
    btnInvisible
    bind:close
    {offsetTop}
    {offsetLeft}
    {onToggle}
    {onLeft}
    {onRight}
    {onUp}
    {onDown}
>
    {#snippet button()}
        <div class="btn" data-border={!borderless}>
            <div class="btnText" title={ariaLabel}>
                {value}
            </div>
            <IconChevronDown width=".8rem" />
        </div>
    {/snippet}

    <div role="listbox" tabindex="0" class="popover" style:max-height={maxHeight} {onkeydown}>
        {#if withSearch}
            <SearchBar
                bind:value={searchValue}
                bind:focus={focusSearch}
                onFocus={() => (selected = -1)}
            />
        {/if}

        <div bind:this={refOptions} class="popoverOptions">
            {#each optionsFiltered as option, i}
                <Button invisible invisibleOutline onclick={ev => select(ev, option)}>
                    <div
                        class="optPopover"
                        aria-selected={value === option}
                        data-focus={selected === i}
                    >
                        {option}
                    </div>
                </Button>
            {/each}
        </div>
    </div>
    <input type="hidden" {name} aria-hidden="true" bind:value />
</Popover>

<style>
    .btn {
        padding: 0.15rem;
        display: inline-flex;
        color: hsl(var(--text));
        border-radius: var(--border-radius);
        font-weight: normal;
        font-size: 0.9rem;
        transition: all 150ms;
    }

    .btn[data-border='true'] {
        padding: 0.15rem 0.33rem 0.3rem 0.5rem;
        border: 1px solid hsla(var(--text) / 0.2);
        border-radius: var(--border-radius);
    }

    .btnText {
        margin: 0 0.25rem;
        text-wrap: nowrap;
    }

    .btn:hover {
        color: hsl(var(--action));
    }

    .optPopover {
        width: 100%;
        text-align: left;
        padding: 0.25rem 0.7rem;
        color: hsl(var(--text));
        font-weight: normal;
        border-radius: var(--border-radius);
        cursor: pointer;
        transition: all 150ms;
    }

    .optPopover[aria-selected='true'] {
        color: hsl(var(--text-high));
    }

    .optPopover:hover {
        color: hsl(var(--action));
    }

    .optPopover[data-focus='true'] {
        color: hsl(var(--action));
        background: hsl(var(--bg-high));
    }

    .popover {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .popoverOptions {
        /*height: 100%;*/
        width: 100%;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
    }
</style>
