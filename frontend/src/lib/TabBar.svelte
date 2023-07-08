<script>
    import Button from "./Button.svelte";

    export let labels = [];
    export let level = 3;
    export let selected;
    export let center = false;
    export let width = "inherit";

    let toggle = [];

    $: if (selected) {
        toggles();
    }

    function toggles() {
        let toggles = [];
        for (let label of labels) {
            toggles.push(selected === label);
        }
        toggle = toggles;
    }

</script>

<div class="bar noselect" style:justify-content={center ? 'space-evenly' : 'flex-start'}>
    {#each labels as label, i}
        <Button
                bind:level
                bind:width
                bind:selected={toggle[i]}
                on:click={() => selected = label}
                on:keypress={() => selected = label}
                {...$$restProps}
        >
            {label}
        </Button>
    {/each}
</div>

<style>
    .bar {
        margin: 5px 10px 10px 10px;
        display: flex;
        align-items: center;
        flex-wrap: wrap;
    }
</style>
