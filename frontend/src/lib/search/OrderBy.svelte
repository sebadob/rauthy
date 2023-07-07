<script>
    import {onMount} from "svelte";
    import IconBarsArrowDown from "../icons/IconBarsArrowDown.svelte";
    import IconBarsArrowUp from "../icons/IconBarsArrowUp.svelte";
    import Tooltip from "../Tooltip.svelte";

    export let items = [];
    export let resItems;
    export let options = [];
    export let firstDirReverse = false;

    let selected = '';
    let direction = 1;
    let callback;

    onMount(() => {
        if (options.length > 0) {
            selected = options[0].label;
            extractCallback();
        }
        if (firstDirReverse) {
            switchDirection();
        }
        orderItems();
    });

    $: if (items) {
        orderItems();
    }

    $: if (selected) {
        extractCallback();
        orderItems();
    }

    function extractCallback() {
        for (let opt of options) {
            if (opt.label === selected) {
                callback = opt.callback;
                break;
            }
        }

        if (!callback) {
            console.error('Could not find a valid callback function in order options for label ' + selected);
        }
    }

    function orderItems() {
        if (callback) {
            let sorted = [...items];
            sorted.sort((a, b) => {
                return callback(a, b) * direction;
            });
            resItems = [...sorted];
        }
    }

    function switchDirection() {
        direction *= -1;
        orderItems();
    }
</script>

<div class="container">
    {#if options.length > 1}
        <Tooltip text="Order by" yOffset={-30}>
            <select class="opts" bind:value={selected}>
                {#each options as opt}
                    <option value={opt.label}>{opt.label}</option>
                {/each}
            </select>
        </Tooltip>
    {/if}

    {#if options.length > 0}
        <div
                role="button"
                tabindex="0"
                class="icon"
                on:click={switchDirection}
                on:keypress={switchDirection}
        >
            {#if direction === 1}
                <IconBarsArrowUp/>
            {:else}
                <IconBarsArrowDown/>
            {/if}
        </div>
    {/if}
</div>

<style>
    .opts {
        margin-right: 15px;
    }

    .container {
        width: 100%;
        display: flex;
        align-items: center;
    }

    .icon {
        margin-top: 5px;
        cursor: pointer;
        color: var(--col-act2a)
    }

    select {
        padding: 4px;
        color: var(--col-text);
        background: var(--col-bg);
        font-size: 1.05rem;
        border-radius: 5px;
        cursor: pointer;
        border: 1px solid var(--col-glow);
        box-shadow: 1px 1px 2px var(--col-gmid);
    }
</style>
