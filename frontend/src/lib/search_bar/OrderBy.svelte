<script lang="ts">
    import IconBarsArrowDown from '#icons/IconBarsArrowDown.svelte';
    import IconBarsArrowUp from '#icons/IconBarsArrowUp.svelte';
    import Button from '#lib/button/Button.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import Options from '#lib/Options.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { untrack } from 'svelte';

    let {
        ref = $bindable(),
        onChange,
        options = [],
        borderless,
        firstDirReverse,
    }: {
        ref?: undefined | HTMLButtonElement;
        onChange: (option: string, direction: 'up' | 'down') => void;
        options: string[];
        borderless?: boolean;
        firstDirReverse?: boolean;
    } = $props();

    let ta = useI18nAdmin();

    let selected = $state(untrack(() => options[0]));
    let direction: 'up' | 'down' = $state(untrack(() => (firstDirReverse ? 'down' : 'up')));
    let directionLabel = $derived(
        direction === 'up' ? ta.search.orderChangeToDesc : ta.search.orderChangeToAsc,
    );

    $effect(() => {
        onChange(selected, direction);
    });
</script>

<div class="container admin-sort">
    <Tooltip text={ta.search.orderBy}>
        <Options
            bind:ref
            ariaLabel={ta.search.orderBy}
            {options}
            bind:value={selected}
            {borderless}
            showTitle={false}
        />
    </Tooltip>

    <div class="btn">
        <Tooltip text={directionLabel}>
            <Button
                ariaLabel={directionLabel}
                invisible
                onclick={() => (direction = direction === 'up' ? 'down' : 'up')}
            >
                {#if direction === 'up'}
                    <IconBarsArrowUp />
                {:else}
                    <IconBarsArrowDown />
                {/if}
            </Button>
        </Tooltip>
    </div>
</div>

<style>
    .btn {
        margin: 0.25rem 1rem -0.25rem 0.5rem;
    }

    .container {
        display: flex;
        align-items: center;
    }

    .admin-sort {
        justify-content: flex-end;
    }
    .admin-sort > :global(.btn) {
        margin: 0;
    }
    .admin-sort :global(.admin-select-trigger) {
        min-height: 32px;
        border: 0;
        padding: 0 6px;
        background: transparent;
    }
    .admin-sort :global(svg) {
        width: 18px;
        height: 18px;
    }
    .admin-sort :global(.admin-select-trigger[data-border]) {
        border: 0;
        background: transparent;
        padding: 0 6px;
    }
</style>
