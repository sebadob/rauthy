<script>
    export let labels = [];
    export let selected;
</script>

<div class="bar noselect">
    {#each labels as label}
        <button
                class="btn font-label"
                data-selected={selected === label}
                on:click={() => selected = label}
                on:keypress={() => selected = label}
        >
            {label}
        </button>
    {/each}
</div>

<style>
    .bar {
        margin: 5px 10px 10px 10px;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        flex-wrap: wrap;
    }

    .btn {
        padding: .25rem .5rem;
        background: transparent;
        border: none;
        border-bottom: 1px solid var(--col-gmid);
        color: var(--col-act2a);
        font-size: .95rem;
        cursor: pointer;
    }

    .btn:hover {
        color: var(--col-act2);
        border-bottom: 1px solid var(--col-act2);
    }

    .btn[data-selected="true"] {
        font-weight: bold;
        color: var(--col-act2);
        border-bottom: 1px solid var(--col-act2);
    }
</style>
