<script>
    import Button from "$lib/Button.svelte";

    export let t;
    export let selected = t.navInfo;
    export let showWebId = false;
    export let showWide = false;

    let labels = showWebId ?
        [t.navInfo, t.navEdit, t.navPassword, t.navMfa, 'WebID', t.navLogout]
        : [t.navInfo, t.navEdit, t.navPassword, t.navMfa, t.navLogout];
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

<div class={showWide ? 'containerWide noselect' : 'container noselect'}>
    {#each labels as label, i}
        <Button
                level={3}
                width={showWide ? 'inherit' : '6rem'}
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
    .container {
        margin-right: 10px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
    }

    .containerWide {
        margin-left: 5px;
        display: flex;
        flex-direction: row;
        align-items: center;
        flex-wrap: wrap;
    }
</style>
