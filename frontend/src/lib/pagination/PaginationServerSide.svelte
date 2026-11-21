<script lang="ts">
    import PaginationControls from './PaginationControls.svelte';
    import { loadPageSize, PAGE_SIZE_DEFAULT, savePageSize, type PageSize } from './props.ts';
    import { untrack } from 'svelte';

    let {
        pageSize = $bindable(loadPageSize()),
        sspFetch,
        idxTotalCount,
        itemsLength,
        firstFetchHeaders,
    }: {
        pageSize: PageSize;
        sspFetch: (urlParams: string) => Promise<[number, Headers]>;
        idxTotalCount?: string;
        itemsLength: number;
        firstFetchHeaders: Headers;
    } = $props();

    let page = $state(1);
    let pageCount = $state(1);
    let continuationToken: string | null = $state(null);
    let itemsTotal: number | undefined = $state();
    let pending = $state(false);
    let loadedHeaders = untrack(() => firstFetchHeaders);

    function readHeaders(headers: Headers) {
        loadedHeaders = headers;
        pageSize = Number(headers.get('x-page-size') || PAGE_SIZE_DEFAULT) as PageSize;
        pageCount = Number(headers.get('x-page-count') || 1);
        continuationToken = headers.get('x-continuation-token');
        const total = idxTotalCount ? headers.get(idxTotalCount) : null;
        itemsTotal = total === null ? undefined : Number(total);
    }

    untrack(() => readHeaders(firstFetchHeaders));

    $effect(() => {
        const headers = firstFetchHeaders;
        if (headers !== loadedHeaders && !untrack(() => pending)) {
            untrack(() => {
                // Refreshes issued by the list itself load its first page.
                readHeaders(headers);
                page = 1;
            });
        }
    });

    async function navigate(target: number, size = pageSize) {
        if (pending || target < 1 || target > pageCount) return;
        const params = new URLSearchParams({ page_size: size.toString() });
        if (target !== 1) {
            if (!continuationToken) return;
            params.set('continuation_token', continuationToken);
            if (target < page) {
                params.set('backwards', 'true');
                params.set('offset', String(Math.max(0, itemsLength - 1)));
            }
        }
        pending = true;
        try {
            const [status, headers] = await sspFetch(params.toString());
            if (status === 206) {
                readHeaders(headers);
                page = target;
            }
        } finally {
            pending = false;
        }
    }
</script>

<PaginationControls
    {page}
    {pageCount}
    {pageSize}
    total={itemsTotal}
    {pending}
    onPage={navigate}
    onPageSize={async size => {
        await navigate(1, size);
        if (pageSize === size) savePageSize(size);
    }}
/>
