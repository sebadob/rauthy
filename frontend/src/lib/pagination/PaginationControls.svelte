<script lang="ts">
    import IconChevronRight from '#icons/IconChevronRight.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { PageSize } from './props.ts';

    let {
        page,
        pageCount,
        pageLinks,
        pageSize,
        total,
        compact,
        pending,
        onPage,
        onPageSize,
    }: {
        page: number;
        pageCount: number;
        pageLinks?: number[];
        pageSize: PageSize;
        total?: number;
        compact?: boolean;
        pending?: boolean;
        onPage: (page: number) => void;
        onPageSize: (size: PageSize) => void;
    } = $props();

    const t = useI18n();
    const sizes = [5, 10, 15, 20, 30, 50, 100] as const;
</script>

<nav aria-label={t.pagination.pagination} aria-busy={pending} class="pagination">
    <div class="controls">
        <button
            type="button"
            aria-label={t.pagination.gotoPagePrev}
            disabled={pending || page <= 1}
            onclick={() => onPage(page - 1)}
        >
            <span class="previous"><IconChevronRight width="1rem" /></span>
        </button>
        <ul>
            {#each pageLinks ?? [page] as number (number)}
                <li
                    aria-label={`${t.pagination.gotoPage} ${number}`}
                    aria-current={page === number ? 'page' : undefined}
                >
                    {#if pageLinks}
                        <button type="button" onclick={() => onPage(number)}>{number}</button>
                    {:else}
                        <span class="current">{number}</span>
                    {/if}
                </li>
            {/each}
        </ul>
        <button
            type="button"
            aria-label={t.pagination.gotoPageNext}
            disabled={pending || page >= pageCount}
            onclick={() => onPage(page + 1)}
        >
            <IconChevronRight width="1rem" />
        </button>
    </div>
    {#if !compact}
        <div class="details">
            <label>
                {t.pagination.entries}
                <select
                    aria-label={t.pagination.showCount}
                    value={pageSize}
                    disabled={pending}
                    onchange={event => {
                        const next = Number(event.currentTarget.value) as PageSize;
                        event.currentTarget.value = String(pageSize);
                        onPageSize(next);
                    }}
                >
                    {#each sizes as size (size)}
                        <option value={size}>{size}</option>
                    {/each}
                </select>
            </label>
            {#if total !== undefined}
                <span>{t.pagination.total}: {total}</span>
            {/if}
        </div>
    {/if}
</nav>

<style>
    .pagination {
        --control-size: 1.75rem;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: space-between;
        gap: 0.375rem 0.75rem;
        flex-shrink: 0;
        padding-block-start: 0.5rem;
        border-top: 1px solid var(--ui-outline);
        font-size: 0.75rem;
        line-height: var(--ui-control-line-height);
        color: var(--ui-muted);
    }

    .controls,
    ul,
    .details,
    label {
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    .controls,
    ul {
        gap: 0.125rem;
    }

    ul {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    li {
        margin: 0;
    }

    button,
    .current,
    select {
        box-sizing: border-box;
        display: grid;
        place-items: center;
        height: var(--control-size);
        min-width: var(--control-size);
        margin: 0;
        padding: 0 0.25rem;
        border: 1px solid transparent;
        border-radius: var(--border-radius);
        background: transparent;
        color: inherit;
        font: inherit;
    }

    button,
    select {
        cursor: pointer;
    }

    button:hover:not(:disabled),
    select:hover:not(:disabled) {
        background-color: var(--ui-hover);
        color: var(--ui-text);
    }

    button:focus-visible,
    select:focus-visible {
        outline: 2px solid var(--ui-primary);
        outline-offset: 2px;
    }

    button:disabled,
    select:disabled {
        cursor: default;
        opacity: 0.45;
    }

    li[aria-current='page'] button,
    .current {
        background: var(--ui-tonal);
        color: var(--ui-primary);
        font-weight: 600;
    }

    .previous {
        display: grid;
        transform: rotate(180deg);
    }

    button :global(svg) {
        display: block;
        width: 1rem;
        height: 1rem;
    }

    .details {
        flex: 1 1 auto;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: 0.25rem 0.75rem;
    }

    label {
        gap: 0.5rem;
    }

    select {
        border-color: var(--ui-outline);
        background-color: var(--ui-surface);
        color: var(--ui-text);
    }

    option {
        background: var(--ui-surface);
        color: var(--ui-text);
    }
</style>
