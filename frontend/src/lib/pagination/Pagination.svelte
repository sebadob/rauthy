<script lang="ts">
    import IconChevronRight from '#icons/IconChevronRight.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { untrack } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import Options from '#lib/Options.svelte';
    import type { PageSize } from '#lib/pagination/props.ts';

    const options: PageSize[] = [5, 10, 20, 30, 50, 100];

    let {
        items = $bindable(),
        itemsPaginated = $bindable(),
        page = $bindable(1),
        pageSize = $bindable(options[2]),
        compact = false,
    }: {
        items: any[];
        itemsPaginated: any[];
        page?: number;
        pageSize?: PageSize;
        compact?: boolean;
    } = $props();

    let t = useI18n();

    const iconSize = '1rem';
    let pageSizeBefore = untrack(() => pageSize);

    let itemsArr: any[] = $state([]);
    let pageLinks: number[] = $state([]);

    $effect(() => {
        // make sure to reset and go to page 1 if chunk size changes
        if (pageSize !== pageSizeBefore) {
            pageSizeBefore = untrack(() => pageSize);
            page = 1;
        }
    });

    $effect(() => {
        let res = [];
        for (let i = 0; i < items.length; i += pageSize) {
            const chunk = items.slice(i, i + pageSize);
            res.push(chunk);
        }
        itemsArr = res;
        page = Math.max(1, Math.min(page, res.length || 1));
        itemsPaginated = res[page - 1] || [];
    });

    $effect(() => {
        calculate();
    });

    function goLeft() {
        if (page > 1) {
            goto(page - 1);
        }
    }

    function goRight() {
        if (page < itemsArr.length) {
            goto(page + 1);
        }
    }

    function goto(newPage: number) {
        page = newPage;
        calculate();
    }

    function calculate() {
        let links = [];
        const visiblePages = 5;
        let countHalf = Math.floor(visiblePages / 2);

        if (itemsArr.length <= visiblePages) {
            for (let i = 1; i <= itemsArr.length; i++) {
                links.push(i);
            }
        } else if (page <= countHalf) {
            for (let i = 1; i <= visiblePages; i++) {
                links.push(i);
            }
        } else if (page > itemsArr.length - countHalf - 1) {
            for (let i = itemsArr.length - visiblePages; i <= itemsArr.length - 1; i++) {
                links.push(i + 1);
            }
        } else {
            for (let i = page - countHalf; i < page - countHalf + visiblePages; i++) {
                links.push(i);
            }
        }

        pageLinks = links;
    }
</script>

{#snippet links()}
    <nav aria-label={t.pagination.pagination}>
        <ul>
            {#each pageLinks as no}
                <li
                    aria-label={`${t.pagination.gotoPage} ${no}`}
                    aria-current={page === no ? 'step' : undefined}
                >
                    <Button invisible onclick={() => goto(no)} onLeft={goLeft} onRight={goRight}>
                        <div class="link noselect">
                            {no}
                        </div>
                    </Button>
                </li>
            {/each}
        </ul>
    </nav>
{/snippet}

{#snippet chunkSizeTotal()}
    <div class="flex gap-10">
        <div class="flex gap-05 chunkSize noselect">
            <div>
                {t.pagination.entries}
            </div>
            <div>
                <Options
                    ariaLabel={t.pagination.showCount}
                    bind:value={pageSize}
                    {options}
                    offsetTop="-14rem"
                    borderless
                />
            </div>
        </div>
        <div class="font-label total">{t.pagination.total}: {items.length}</div>
    </div>
{/snippet}

<div class="container admin-pagination">
    <div class="flex">
        <Button
            ariaLabel={t.pagination.gotoPagePrev}
            onclick={goLeft}
            invisible
            isDisabled={page === 1}
        >
            <div class="iconLeft" aria-label={t.pagination.gotoPagePrev} data-disabled={page === 1}>
                <IconChevronRight width={iconSize} />
            </div>
        </Button>

        {@render links()}

        <Button
            ariaLabel={t.pagination.gotoPageNext}
            onclick={goRight}
            invisible
            isDisabled={page >= itemsArr.length}
        >
            <div
                class="iconRight"
                aria-label={t.pagination.gotoPageNext}
                data-disabled={page === itemsArr.length}
            >
                <IconChevronRight width={iconSize} />
            </div>
        </Button>
    </div>

    {#if !compact}
        {@render chunkSizeTotal()}
    {/if}
</div>

<style>
    nav {
        display: flex;
        margin: 0 0.8rem 0 0.25rem;
    }

    ul {
        list-style-type: none;
        margin: 0;
        padding: 0;
        display: inline-flex;
    }

    li {
        margin: 0;
    }

    li[aria-current='step'] div {
        color: hsla(var(--action) / 0.65);
        font-weight: bold;
        text-decoration: underline;
    }

    .chunkSize {
        margin-left: 10px;
    }

    .container {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        padding: 0.25rem 0;
    }

    .iconLeft[data-disabled='true'],
    .iconRight[data-disabled='true'] {
        color: hsl(var(--bg-high));
    }

    .iconLeft {
        margin: -5px 0 0 0;
        transform: rotate(180deg);
    }

    .iconRight {
        margin: 0 0 -6px -8px;
    }

    .link {
        width: 1.2rem;
        margin: 2px;
        padding: 2px;
        cursor: pointer;
        color: hsla(var(--action) / 0.5);
    }

    .total {
        transform: translateY(0.05rem);
        text-wrap: nowrap;
    }

    :global(.admin-shell) .admin-pagination {
        display: flex;
        flex-wrap: wrap;
        gap: 12px;
        flex-shrink: 0;
        margin-top: 20px;
        padding: 16px 0 0;
        border-top: 1px solid var(--ui-outline);
        font-size: 12px;
    }
    :global(.admin-shell) .admin-pagination > :global(.flex) {
        gap: 4px;
    }
    :global(.admin-shell) .admin-pagination > :global(.flex:last-child) {
        width: 100%;
        justify-content: space-between;
    }
    :global(.admin-shell) .admin-pagination :global(nav),
    :global(.admin-shell) .admin-pagination :global(ul),
    :global(.admin-shell) .admin-pagination :global(.chunkSize) {
        margin: 0;
    }
    :global(.admin-shell) .admin-pagination :global(.link) {
        width: auto;
        margin: 0;
        padding: 0;
        color: inherit;
        font-size: 13px;
        text-decoration: none;
    }
    :global(.admin-shell)
        .admin-pagination
        :global(li[aria-current='step'])
        :global(.admin-control) {
        background: var(--ui-tonal);
        color: var(--ui-primary);
    }
    :global(.admin-shell) .admin-pagination :global(.total) {
        font-size: 12px;
        transform: none;
    }
    :global(.admin-shell) .admin-pagination :global(.admin-control[data-variant]) {
        width: 32px;
        min-width: 32px;
        height: 32px;
        min-height: 32px;
        padding: 0;
        border-radius: 50%;
    }
</style>
