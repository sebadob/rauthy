<script lang="ts">
    import { genKey } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import IconMagnify from '#icons/IconMagnify.svelte';
    import Options from '#lib/Options.svelte';
    import IconBackspace from '#icons/IconBackspace.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';

    let t = useI18n();
    let ta = useI18nAdmin();

    let {
        ref = $bindable(),
        id = genKey(),
        ariaLabel = t.common.search,
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
        ref?: undefined | HTMLElement;
        id?: string;
        ariaLabel?: string;
        value?: string;
        placeholder?: string;
        datalist?: string[];
        options?: string[];
        option?: string;
        pattern?: string;
        focus?: () => void;
        width?: string;
        borderless?: boolean;
        onSearch?: (value: string, option?: string) => void;
        onTab?: (value: string) => void;
        onUp?: (value: string) => void;
        onDown?: (value: string) => void;
        onFocus?: () => void;
    } = $props();

    const idDatalist = genKey();

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
    class="flex container admin-search"
    style:--search-border={borderless ? '0' : '1px solid hsl(var(--bg-high))'}
    style:--search-width={width}
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

    <div class="relative clear-slot" class:empty={!value}>
        <div class="absolute btnDelete">
            <Button ariaLabel={t.common.delete} invisible onclick={() => (value = '')}>
                <span class="backspace">
                    <IconBackspace color="hsla(var(--text) / .5)" />
                </span>
            </Button>
        </div>
    </div>

    {#if onSearch}
        <div class="btnSearch">
            <Button ariaLabel={t.common.search} invisible onclick={search}>
                <IconMagnify />
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
        bottom: 0.2rem;
    }

    .btnDelete {
        top: -0.55rem;
        right: 0.3rem;
        opacity: 0.8;
    }

    .btnSearch {
        margin: 2px 2px 2px 0;
        padding: 0 0.25rem;
        background: hsl(var(--bg-high));
        border-left: 1px solid hsl(var(--bg-high));
        border-radius: 0 2px 2px 0;
    }

    .container {
        width: var(--search-width);
        border: var(--search-border);
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .options {
        margin-left: 0.1rem;
        background: hsl(var(--bg-high));
        border-right: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius) 0 0 var(--border-radius);
    }

    :global(.admin-shell) .admin-search:not(:global(.branding-preview *)) {
        display: flex;
        align-items: center;
        width: 100%;
        min-width: 0;
        height: var(--ui-search-height);
        margin: 0;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        overflow: hidden;
    }
    :global(.admin-shell) .admin-search:focus-within:not(:global(.branding-preview *)) {
        border-color: var(--ui-primary);
        outline: 2px solid hsl(var(--action) / 0.16);
    }
    :global(.admin-shell) .admin-search:not(:global(.branding-preview *)) input[type='search'] {
        flex: 1;
        min-width: 0;
        height: 100%;
        padding: 0 0.625rem;
        border: 0;
        border-radius: 0;
        outline: 0;
        background: transparent;
        font-family: inherit;
        font-size: var(--ui-control-font-size);
        line-height: var(--ui-control-line-height);
        appearance: none;
    }

    :global(.admin-shell) .admin-search input[type='search']::-webkit-search-cancel-button {
        appearance: none;
    }
    :global(.admin-shell) .admin-search:not(:global(.branding-preview *)) :global(.options) {
        margin: 0;
        background: transparent;
        border: 0;
        border-right: 1px solid var(--ui-outline);
        border-radius: 0;
    }
    :global(.admin-shell) .admin-search .clear-slot {
        display: flex;
        align-items: center;
        flex: none;
    }

    :global(.admin-shell) .admin-search .clear-slot.empty {
        visibility: hidden;
    }

    :global(.admin-shell) .admin-search .btnDelete {
        position: static;
    }

    :global(.admin-shell) .admin-search .backspace {
        display: flex;
        bottom: auto;
    }
    :global(.admin-shell)
        .admin-search:not(:global(.branding-preview *))
        :global(.admin-select-trigger) {
        padding: 0 10px;
    }
</style>
