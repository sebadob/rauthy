<script>
    import OptionSelect from "./OptionSelect.svelte";
    import IconChevronRight from "./icons/IconChevronRight.svelte";

    export let itemsTotal = 0;
    export let sspPage = 1;
    export let sspPageSize = 15;
    export let sspContinuationToken = '';
    export let fetchPageCallback = (offset, backwards) => {
    };

    const options = [2, 3, 5, 7, 10, 15, 20, 30, 50, 100];
    const iconSize = 16;

    let pageSize = sspPageSize;

    $: if (pageSize) {
        console.log('page size change in pagination server');
        fetchPageCallback(0, false)
    }

</script>

<div class="container">
    {#if sspPage !== 1}
        <div
                role="button"
                tabindex="0"
                class="icon iconLeft"
                on:click={() => fetchPageCallback(0, true)}
                on:keypress={() => fetchPageCallback(0, true)}
        >
            <IconChevronRight width={iconSize}/>
        </div>
    {/if}

    <div class="links">
    </div>

    {#if sspContinuationToken}
        <div
                role="button"
                tabindex="0"
                class="icon iconRight"
                on:click={() => fetchPageCallback(0, false)}
                on:keypress={() => fetchPageCallback(0, false)}
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

    <div class="font-label total">Total: {itemsTotal}</div>
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

    .total {
        margin: .2rem 0 0 .5rem;
        font-weight: bold;
        font-size: .9rem;
    }
</style>
