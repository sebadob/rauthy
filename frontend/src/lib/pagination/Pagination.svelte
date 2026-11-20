<script lang="ts" generics="T">
    import PaginationControls from './PaginationControls.svelte';
    import { PAGE_SIZE_DEFAULT, type PageSize } from './props.ts';

    let {
        items = $bindable(),
        itemsPaginated = $bindable(),
        page = $bindable(1),
        pageSize = $bindable(PAGE_SIZE_DEFAULT),
        compact = false,
    }: {
        items: T[];
        itemsPaginated: T[];
        page?: number;
        pageSize?: PageSize;
        compact?: boolean;
    } = $props();

    const pageCount = $derived(Math.max(1, Math.ceil(items.length / pageSize)));
    const pageLinks = $derived.by(() => {
        const length = Math.min(5, pageCount);
        const start = Math.min(Math.max(page - 2, 1), pageCount - length + 1);
        return Array.from({ length }, (_, index) => start + index);
    });

    $effect(() => {
        const current = Math.max(1, Math.min(page, pageCount));
        if (current !== page) page = current;
        const offset = (current - 1) * pageSize;
        itemsPaginated = items.slice(offset, offset + pageSize);
    });
</script>

<PaginationControls
    {page}
    {pageCount}
    {pageLinks}
    {pageSize}
    total={items.length}
    {compact}
    onPage={value => (page = value)}
    onPageSize={size => {
        page = 1;
        pageSize = size;
    }}
/>
