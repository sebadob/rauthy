<script>
    import AccWebIdEntry from "./AccWebIdEntry.svelte";
    import IconPlus from "$lib/icons/IconPlus.svelte";
    import {onMount} from "svelte";
    import {REGEX_URI_SPACE} from "../../utils/constants.js";

    export let webIdData;
    export let viewModePhone = false;

    let entries;

    $: containsEmptyKey = entries?.length > 0 && entries.filter(e => e.key === '').length > 0;

    onMount(() => {
        let arr = [];
        if (webIdData.data) {
            for (let [key, value] of Object.entries(webIdData.data)) {
                arr.push({key, value, keyErr: '', valueErr: ''});
            }
        }
        entries = arr;
    });

    function add() {
        entries = [...entries, {key: '', value: '', keyErr: '', valueErr: ''}];
    }

    export function getData() {
        let data;
        if (entries.length > 0) {
            data = {};
            for (let entry of entries) {
                data[entry.key] = entry.value;
            }
        }
        return data;
    }

    function remove(k) {
        entries = [...entries.filter(e => e.key !== k)];
    }

    function isKeyUnique(k) {
        return entries.filter(e => e.key === k).length === 1;
    }

    export function validateData() {
        let isInvalid = false;

        for (let entry of entries) {
            if (!REGEX_URI_SPACE.test(entry.key)) {
                console.log(entry.key + ' is invalid');
                entry.keyErr = 'INVALID';
                isInvalid = true;
            }

            if (!REGEX_URI_SPACE.test(entry.value)) {
                console.log(entry.value + ' is invalid');
                entry.valueErr = 'INVALID';
                isInvalid = true;
            }
        }

        // just to trigger a re-render in case of any new errors
        if (isInvalid) {
            entries = entries;
        }

        return !isInvalid;
    }

</script>

{#if entries}
    {#each entries as entry, c (c)}
<!--                isKeyUnique={isKeyUnique}-->
<!--                removeKey={remove}-->
        <AccWebIdEntry
                removeEntry={remove}
                bind:viewModePhone
                bind:entry
        />
    {/each}
{/if}

{#if !containsEmptyKey}
    <div
            role="button"
            tabindex="0"
            class="btnNew"
            on:click={add}
            on:keypress={add}
    >
        <IconPlus/>
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
