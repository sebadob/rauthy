<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { SelectItem } from '$lib5/select_list/props.ts';
    import Button from '$lib5/button/Button.svelte';
    import IconEdit from '$icons/IconEdit.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Modal from '$lib5/Modal.svelte';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import { useI18n } from '$state/i18n.svelte';

    let {
        items = $bindable(),
        maxWidth = '467pt', // matches <p> max width
        children,
    }: {
        items: SelectItem[];
        maxWidth?: string;
        children: Snippet;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let innerWidth: undefined | number = $state();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let compact = $derived(innerWidth && innerWidth < 800);
    let joined = $derived(
        items
            .filter(i => i.selected)
            .map(i => i.name)
            .join(', '),
    );
    let anySelected = $derived(items.find(i => i.selected));
</script>

<svelte:window bind:innerWidth />

<div class="container" style:max-width={maxWidth}>
    <div class="items">
        {#if anySelected}
            <div class="joined">
                {joined}
            </div>
        {/if}

        <div class="edit">
            <Button invisible onclick={() => (showModal = true)}>
                <div title={ta.common.edit}>
                    <IconEdit width="1.2rem" />
                </div>
            </Button>
        </div>
        <Modal bind:showModal bind:closeModal>
            <h3>{@render children()}</h3>
            {#if compact}
                <div class="compact">
                    {#each items as item (item.name)}
                        <InputCheckbox ariaLabel={item.name} bind:checked={item.selected}>
                            {item.name}
                        </InputCheckbox>
                    {/each}
                </div>
            {:else}
                <div class="boxes">
                    <div>
                        {#each items as item (item.name)}
                            {#if !item.selected}
                                <InputCheckbox ariaLabel={item.name} bind:checked={item.selected}>
                                    {item.name}
                                </InputCheckbox>
                            {/if}
                        {/each}
                    </div>
                    <div>
                        {#each items as item (item.name)}
                            {#if item.selected}
                                <InputCheckbox ariaLabel={item.name} bind:checked={item.selected}>
                                    {item.name}
                                </InputCheckbox>
                            {/if}
                        {/each}
                    </div>
                </div>
            {/if}

            <Button onclick={() => closeModal?.()}>
                {t.common.close}
            </Button>
        </Modal>
    </div>

    <div class="label font-label">
        {@render children()}
    </div>
</div>

<style>
    .boxes {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
    }

    .boxes,
    .compact {
        margin: 1rem 0;
    }

    .boxes > div {
        height: 20rem;
        min-width: 12rem;
        max-width: calc(100dvw - 3rem);
        padding: 0.25rem 0.5rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / 0.2);
        overflow: auto;
    }

    .container {
        margin: 0.5rem 0;
        padding: 0.5rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / 0.1);
        line-height: 1.2rem;
    }

    .edit {
        height: 1rem;
        margin-top: -0.25rem;
    }

    .items {
        min-height: 1.2rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .joined {
        text-wrap: wrap;
        word-break: break-word;
    }

    .label {
        font-size: 0.9rem;
        color: hsla(var(--text) / 0.8);
    }
</style>
