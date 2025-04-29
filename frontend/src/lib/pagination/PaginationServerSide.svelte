<script lang="ts">
    import IconChevronRight from "$icons/IconChevronRight.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Button from "$lib5/button/Button.svelte";
    import Options from "$lib5/Options.svelte";
    import {PAGE_SIZE_DEFAULT, type PageSize} from "$lib5/pagination/props";
    import {onMount, untrack} from "svelte";

    const options: PageSize[] = [5, 10, 20, 30, 50, 100];

    let {
        pageSize = $bindable(PAGE_SIZE_DEFAULT),
        sspFetch,
        idxTotalCount,
        itemsLength,
        firstFetchHeaders,
    }: {
        pageSize: PageSize,
        sspFetch: (urlParams: string) => Promise<[number, Headers]>;
        idxTotalCount?: string,
        itemsLength: number,
        firstFetchHeaders: Headers,
    } = $props();

    let t = useI18n();

    const iconSize = "1rem";

    // We want to keep that logic inside this component instead of expecting
    // the parent to extract the information from the headers.
    pageSize = Number.parseInt(firstFetchHeaders.get('x-page-size') || PAGE_SIZE_DEFAULT.toString()) as PageSize;

    let itemsTotal: undefined | null | number = $state();
    let pageSizeBefore = untrack(() => pageSize);
    let pageCount = $state(Number.parseInt(firstFetchHeaders.get('x-page-count') || '1'));
    let continuationToken: string | undefined | null = $state(firstFetchHeaders.get('x-continuation-token'));
    let page = $state(1);

    let isLastPage = $derived(page >= pageCount);

    onMount(() => {
        if (idxTotalCount) {
            let head = firstFetchHeaders.get(idxTotalCount);
            if (head) {
                itemsTotal = Number.parseInt(head);
            }
        }
    });

    $effect(() => {
        handlePageSizeChange();
    });

    async function handlePageSizeChange() {
        if (pageSize !== pageSizeBefore) {
            pageSizeBefore = pageSize;
            continuationToken = undefined;
            let [status, headers] = await sspFetch(`page_size=${pageSize}`);
            parseHeaders(status, headers);
            page = 1;
        }
    }

    function buildParams(backwards: boolean): string {
        let params = `page_size=${untrack(() => pageSize)}`;
        // let url = `/auth/v1/users?page_size=${sspPageSize}&offset=${sspOffset}`;
        if (backwards) {
            if (page === 2) {
                // if the next page we would fetch is the first one, omit all other params
                return params;
            }
            params += `&backwards=${backwards}&offset=${itemsLength - 1}`;
        }
        if (continuationToken) {
            params += `&continuation_token=${untrack(() => continuationToken)}`;
        }
        return params;
    }

    async function goLeft() {
        if (page > 1) {
            let [status, headers] = await sspFetch(buildParams(true));
            parseHeaders(status, headers);
            page -= 1;
        }
    }

    async function goRight() {
        if (page < pageCount) {
            let [status, headers] = await sspFetch(buildParams(false));
            parseHeaders(status, headers);
            page += 1;
        }
    }

    function parseHeaders(status: number, headers: Headers) {
        if (status === 206) {
            // we get a few headers during SSP we can use for the navigation
            let xPageSize = headers.get('x-page-size');
            if (!xPageSize) {
                console.error('Did not receive x-page-size with SSP');
                return;
            }
            pageSize = Number.parseInt(xPageSize) as PageSize;
            pageCount = Number.parseInt(headers.get('x-page-count') || '1');
            continuationToken = headers.get('x-continuation-token');
            let total = idxTotalCount ? headers.get(idxTotalCount) : undefined;
            // let total = headers.get('x-user-count');
            if (total) {
                itemsTotal = Number.parseInt(total);
            } else {
                itemsTotal = undefined;
            }
        } else {
            console.error('Received non 206 status with SSP');
        }
    }
</script>

<div class="container">
    <div class="flex">
        <Button onclick={goLeft} invisible isDisabled={page < 2}>
            <div class="iconLeft" aria-label={t.pagination.gotoPagePrev} data-disabled={page === 1}>
                <IconChevronRight width={iconSize}/>
            </div>
        </Button>

        <ul>
            <li>
                {page}
            </li>
        </ul>

        <Button onclick={goRight} invisible isDisabled={isLastPage}>
            <div class="iconRight" aria-label={t.pagination.gotoPageNext} data-disabled={isLastPage}>
                <IconChevronRight width={iconSize}/>
            </div>
        </Button>
    </div>

    <div class="flex gap-10">
        <div class="flex gap-05 chunkSize noselect">
            <div>
                {t.pagination.entries}
            </div>
            <div>
                <Options
                        ariaLabel={t.pagination.showCount}
                        bind:value={pageSize}
                        options={options}
                        offsetTop="-14rem"
                        borderless
                />
            </div>
        </div>
        {#if itemsTotal}
            <div class="font-label total">{t.pagination.total}: {itemsTotal}</div>
        {/if}
    </div>
</div>

<style>
    ul {
        list-style-type: none;
        margin: 0;
        padding: 0;
        display: inline-flex;
    }

    li {
        margin: 0;
    }

    .container {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        padding: .25rem 0;
    }

    .iconLeft[data-disabled="true"],
    .iconRight[data-disabled="true"] {
        color: hsl(var(--bg-high));
    }

    .iconLeft {
        margin: -7px 0 0 0;
        transform: rotate(180deg);
    }

    .iconRight {
        margin: 0 0 -6px 0;
    }

    .total {
        transform: translateY(.05rem);
        text-wrap: nowrap;
    }
</style>
