<script>
    import IconMagnify from "$lib/icons/IconMagnify.svelte";
    import {onMount} from "svelte";
    import Tooltip from "../Tooltip.svelte";
    import {getKey} from "../utils/helpers.js";
    import IconBackspace from "$lib/icons/IconBackspace.svelte";
    import {getSearch} from "../../utils/dataFetchingAdmin.js";

    export let items = [];
    export let resItems;
    export let options = [];
    export let useServerSide = false;

    let selected = '';
    let search = '';
    let callback;

    onMount(() => {
        if (options.length > 0) {
            selected = options[0].label;
            extractCallback();
        }
    });

    $: if (selected) {
        extractCallback();
    }

    $: {
        if (!search) {
            resItems = items;
        } else if (useServerSide) {
            filerItemsServerSide();
        } else {
            filerItems();
        }
    }

    function extractCallback() {
        for (let opt of options) {
            if (opt.label === selected) {
                callback = opt.callback;
                break;
            }
        }

        if (!callback) {
            console.error('Could not find a valid callback function in search options for label ' + selected);
        }
    }

    function del() {
        search = '';
    }

    function filerItems() {
        resItems = [...items.filter(i => {
            // This switch is a bit more annoying to maintain, but we can set a more strict CSP without `eval`
            if (options.length > 0) {
                return callback(i, search);
            } else {
                return i.toLowerCase().includes(search) || i === search;
            }
        })];
    }

    async function filerItemsServerSide() {
        if (search.length < 3) {
            // skipping server side search below 3 chars
            return;
        }

        const idx = selected.replaceAll('-', '').toLowerCase();
        let res = await getSearch('user', idx, search);
        if (res.ok) {
            resItems = await res.json();
        } else {
            // should never happen ...
            console.error(res);
        }
    }

</script>

<div class="container">
    {#if options.length > 1}
        <Tooltip text="Search by" yOffset={-30}>
            <select class="opts font-label" bind:value={selected}>
                {#each options as opt}
                    <option value={opt.label}>{opt.label}</option>
                {/each}
            </select>
        </Tooltip>
    {/if}

    <div class="inputBar">
        <input
                class="input"
                type="text"
                name={getKey()}
                bind:value={search}
                placeholder="Search"
                autocomplete="off"
        />
        <div class="magnify">
            <IconMagnify width={20}/>
        </div>
    </div>

    <div class="backWrap">
        <div
                role="button"
                tabindex="0"
                class="back"
                on:click={del}
                on:keypress={del}
        >
            <IconBackspace/>
        </div>
    </div>
</div>

<style>
    .back {
        position: absolute;
        top: -12px;
        right: 8px;
        cursor: pointer;
        color: var(--col-gmid)
    }

    .backWrap {
        position: relative;
    }

    .opts {
        margin-right: 15px;
    }

    .container {
        width: 100%;
        display: flex;
        align-items: center;
    }

    input {
        padding: 5px 30px 5px 25px;
        background: var(--col-bg);
        border: 1px solid var(--col-glow);
        border-radius: 3px;
        color: var(--col-text);
        font-size: 1.05rem;
        outline: none;
        box-shadow: 1px 1px 2px var(--col-gmid);
    }

    input:hover {
        background: white;
    }

    input:focus {
        background: white;
        border: 1px solid var(--col-acnt);
    }

    .inputBar {
        position: relative;
    }

    .magnify {
        position: absolute;
        top: 6px;
        left: 5px;
    }

    select {
        height: 2.13rem;
        color: var(--col-text);
        background: var(--col-bg);
        font-size: 1.05rem;
        border-radius: 3px;
        cursor: pointer;
        border: 1px solid var(--col-glow);
        box-shadow: 1px 1px 2px var(--col-gmid);
    }
</style>
