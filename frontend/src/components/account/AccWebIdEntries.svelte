<script>
    import AccWebIdEntry from "./AccWebIdEntry.svelte";
    import IconPlus from "$lib/icons/IconPlus.svelte";

    export let t;
    export let webIdData;
    export let viewModePhone = false;

    let entries = webIdData.data || [];

    $: containsEmptyKey = entries.length > 0 && entries.filter(e => e.key === '').length > 0;

    function add() {
        entries = [...entries, {
            key: '',
            value: '',
        }];
    }

    function remove(k) {
        entries = [...entries.filter(e => e.key !== k)];
    }

    function isKeyUnique(k) {
        return entries.filter(e => e.key === k).length === 1;
    }

    $: console.log(entries);
    $: console.log(containsEmptyKey);

</script>

{#each entries as entry}
    <AccWebIdEntry
            bind:t
            bind:entry
            isKeyUnique={isKeyUnique}
            removeKey={remove}
            bind:viewModePhone
    />
{/each}

{#if !containsEmptyKey}
    <div
            role="button"
            tabindex="0"
            class="btnNew"
            on:click={add}
            on:keypress={add}
    >
        <IconPlus />
    </div>
{/if}

<style>
    .btnNew {
        margin: .5rem;
        cursor: pointer;
        color: var(--col-act2);
    }

    .btnNew:hover {
        color: var(--col-act2a);
    }
</style>
