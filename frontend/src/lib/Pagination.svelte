<script>
    import {run} from 'svelte/legacy';

    import OptionSelect from "./OptionSelect.svelte";
    import IconChevronRight from "./icons/IconChevronRight.svelte";

    let {items = $bindable([]), resItems = $bindable([])} = $props();

    const options = [5, 7, 10, 15, 20, 30, 50, 100];
    const iconSize = 16;

    let itemsArr = $state([]);
    let page = $state(1);
    let selectorSize = 5;
    let chunkSize = $state(15);
    let showLeft = $state(false);
    let showRight = $state(false);

    let pageLinks = $state([]);

    run(() => {
        if (chunkSize) {
            page = 1;
        }
    });

    run(() => {
        if (chunkSize && items.length > 0) {
            let res = [];
            for (let i = 0; i < items.length; i += chunkSize) {
                const chunk = items.slice(i, i + chunkSize);
                res.push(chunk);
            }
            itemsArr = res;
            resItems = itemsArr[page - 1];
        }
    });

    run(() => {
        if (page) {
            let links = [];
            let countHalf = Math.floor(selectorSize / 2);

            if (itemsArr.length <= selectorSize) {
                for (let i = 1; i <= itemsArr.length; i++) {
                    links.push(i);
                }
                showLeft = false;
                showRight = false;

            } else if (page <= countHalf) {
                for (let i = 1; i <= selectorSize; i++) {
                    links.push(i);
                }
                showLeft = false;
                showRight = true;

            } else if (page > itemsArr.length - countHalf - 1) {
                for (let i = itemsArr.length - selectorSize; i <= itemsArr.length - 1; i++) {
                    links.push(i + 1);
                }
                showLeft = true;
                showRight = false;

            } else {
                for (let i = page - countHalf; i < page - countHalf + selectorSize; i++) {
                    links.push(i);
                }
                showLeft = true;
                showRight = true;
            }

            pageLinks = links;
        }
    });

</script>

<div class="container">
    {#if showLeft}
        <div
                role="button"
                tabindex="0"
                class="icon iconLeft"
                onclick={() => page -= 1}
                onkeypress={() => page -= 1}
        >
            <IconChevronRight width={iconSize}/>
        </div>
    {/if}

    <div class="links">
        {#each pageLinks as no}
            <div
                    role="button"
                    tabindex="0"
                    class="link noselect"
                    class:selected={page === no}
                    onclick={() => page = no}
                    onkeypress={() => page = no}
            >
                {no}
            </div>
        {/each}
    </div>

    {#if showRight}
        <div
                role="button"
                tabindex="0"
                class="icon iconRight"
                onclick={() => page += 1}
                onkeypress={() => page += 1}
        >
            <IconChevronRight width={iconSize}/>
        </div>
    {/if}

    <div class="chunkSize noselect">
        Entries
        <OptionSelect
                bind:value={chunkSize}
                options={options}
                width="50px"
        />
    </div>

    <div class="font-label total">Total: {items.length}</div>
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

    .links {
        display: flex;
        margin: -3px 10px 0 0;
    }

    .link {
        margin: 2px;
        padding: 2px;
        cursor: pointer;
        color: var(--col-act2);
    }

    .selected {
        font-weight: bold;
        text-decoration: underline;
    }

    .total {
        margin: .2rem 0 0 .5rem;
        font-weight: bold;
        font-size: .9rem;
    }
</style>
