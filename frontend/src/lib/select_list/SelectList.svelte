<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { SelectItem } from '#lib/select_list/props.ts';
    import Button from '#lib/button/Button.svelte';
    import IconEdit from '#icons/IconEdit.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import Modal from '#lib/Modal.svelte';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import SearchBar from '#lib/search_bar/SearchBar.svelte';

    let {
        items = $bindable(),
        maxWidth = '467pt', // matches <p> max width
        disabledItems = [],
        children,
    }: {
        items: SelectItem[];
        maxWidth?: string;
        // names of items that are rendered read-only and cannot be toggled
        disabledItems?: string[];
        children: Snippet;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();
    let search = $state('');

    let selected = $derived(items.filter(item => item.selected));
    let filter = $derived(search.trim().toLowerCase());
    let readonly = $derived(items.length > 0 && items.every(i => disabledItems.includes(i.name)));

    function open() {
        search = '';
        showModal = true;
    }
</script>

<div class="admin-selection-field" style:max-width={maxWidth}>
    <div class="label font-label admin-field-label">
        {@render children()}
    </div>
    <div class="selection">
        {#if selected.length}
            <ul class="chips" role="list">
                {#each selected as item (item.name)}
                    <li>{item.name}</li>
                {/each}
            </ul>
        {/if}
        {#if !readonly}
            <Button ariaLabel={ta.common.edit} invisible onclick={open}>
                <div title={ta.common.edit}>
                    <IconEdit width="1.125rem" />
                </div>
            </Button>
        {/if}
    </div>
</div>

<Modal bind:showModal bind:closeModal>
    <div class="picker">
        <h3>{@render children()}</h3>
        {#if items.length > 8}
            <SearchBar bind:value={search} width="100%" />
        {/if}
        <div class="choices">
            {#each items as item (item.name)}
                {#if !filter || item.name.toLowerCase().includes(filter)}
                    <InputCheckbox
                        ariaLabel={item.name}
                        bind:checked={item.selected}
                        disabled={disabledItems.includes(item.name)}
                    >
                        {item.name}
                    </InputCheckbox>
                {/if}
            {/each}
        </div>
        <div class="actions">
            <Button onclick={() => closeModal?.()}>{t.common.close}</Button>
        </div>
    </div>
</Modal>

<style>
    .admin-selection-field {
        min-width: 0;
        margin-block-end: var(--ui-field-margin, var(--ui-space));
    }

    .selection,
    .choices {
        border: 1px solid var(--ui-outline, hsl(var(--bg-high)));
        border-radius: var(--border-radius);
        background: var(--ui-surface, hsl(var(--bg)));
    }

    .selection {
        min-height: var(--ui-field-height, 44px);
        display: flex;
        align-items: center;
        justify-content: end;
        gap: 0.5rem;
        padding: 3px 6px 3px 10px;
    }

    .chips {
        display: flex;
        flex: 1;
        flex-wrap: wrap;
        gap: 4px;
        margin: 0;
        padding: 0;
        list-style: none;
        min-width: 0;
    }

    .chips li {
        margin: 0;
        padding: 3px 8px;
        border-radius: var(--border-radius);
        background: var(--ui-tonal, hsl(var(--bg-high)));
        font-size: 0.8125rem;
        overflow-wrap: anywhere;
    }

    .picker {
        width: min(32rem, calc(100dvw - 6rem - 2px));
        min-width: 0;
        display: grid;
        gap: 0.75rem;
    }

    h3 {
        font-size: 1rem;
        margin: 0;
    }

    .choices {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 13rem), 1fr));
        align-content: start;
        gap: 0.25rem 0.5rem;
        max-height: min(24rem, 55dvh);
        overflow: auto;
        padding: 0.5rem;
    }

    .choices :global(label) {
        min-width: 0;
        padding: 0.125rem 0.5rem;
        border-radius: var(--border-radius);
        overflow-wrap: anywhere;
    }

    .choices :global(label:has(:checked)) {
        background: var(--ui-tonal, hsl(var(--bg-high)));
    }

    .actions {
        display: flex;
        justify-content: end;
    }
</style>
