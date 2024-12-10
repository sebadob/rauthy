<script>
    import { run } from 'svelte/legacy';

    import OptionSelect from "./OptionSelect.svelte";
    import IconChevronRight from "./icons/IconChevronRight.svelte";

    /**
     * @typedef {Object} Props
     * @property {number} [itemsTotal]
     * @property {number} [sspPage]
     * @property {number} [sspPageSize]
     * @property {string} [sspContinuationToken]
     * @property {any} [fetchPageCallback]
     * @property {any} [sspPageSizeChange]
     */

    /** @type {Props} */
    let {
        itemsTotal = 0,
        sspPage = 1,
        sspPageSize = 15,
        sspContinuationToken = '',
        fetchPageCallback = (offset, backwards) => {
    },
        sspPageSizeChange = (pageSize) => {
    }
    } = $props();

    const options = [2, 3, 5, 7, 10, 15, 20, 30, 50, 100];
    const iconSize = 16;

    let pageSize = $state(sspPageSize);

    run(() => {
        if (pageSize) {
            console.log('page size change in pagination server');
            sspPageSizeChange(pageSize);
        }
    });

</script>

<div class="container">
    {#if sspPage !== 1}
        <div
                role="button"
                tabindex="0"
                class="icon iconLeft"
                onclick={() => fetchPageCallback(0, true)}
                onkeypress={() => fetchPageCallback(0, true)}
        >
            <IconChevronRight width={iconSize}/>
        </div>
    {/if}

    <div class="links">
        <div class="pageNo noselect">
            {sspPage}
        </div>
    </div>

    {#if sspContinuationToken}
        <div
                role="button"
                tabindex="0"
                class="icon iconRight"
                onclick={() => fetchPageCallback(0, false)}
                onkeypress={() => fetchPageCallback(0, false)}
        >
            <IconChevronRight width={iconSize}/>
        </div>
    {/if}

    <div class="chunkSize noselect">
        Entries
        <OptionSelect
                bind:value={pageSize}
                options={options}
                width="50px"
        />
    </div>

    {#if itemsTotal && itemsTotal > 0}
        <div class="font-label total">Total: {itemsTotal}</div>
    {/if}
</div>

<style>
    .chunkSize {
        margin-left: 10px;
    }

    .container {
        display: flex;
        align-items: center;
        padding: .25rem .5rem;
    }

    .icon {
        cursor: pointer;
        color: var(--col-act2);
    }

    .iconRight {
        margin: 0 0 -3px -8px;
    }

    .iconLeft {
        margin: -5px 0 0 0;
        transform: rotate(180deg);
    }

    /* matches the name in the client side pagination, even though no links are here */
    .links {
        display: flex;
        margin: -3px 10px 0 0;
    }

    .pageNo {
        margin: 2px;
        padding: 2px;
        color: var(--col-acnt);
    }

    .total {
        margin: .2rem 0 0 .5rem;
        font-weight: bold;
        font-size: .9rem;
    }
</style>
