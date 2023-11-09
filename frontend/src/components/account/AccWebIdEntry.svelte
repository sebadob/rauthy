<script>
    import {REGEX_URI_SPACE} from "../../utils/constants.js";
    import Input from "$lib/inputs/Input.svelte";
    import IconStop from "$lib/icons/IconStop.svelte";

    export let t;
    export let entry = { key: '', value: '', keyErr: '', valueErr: '' };

    export let isKeyUnique;
    export let removeKey;
    export let viewModePhone = false;

    $: inputWidth = viewModePhone ? 'calc(100vw - 3.5rem)' : '360px';
    $: inputWidthValue = viewModePhone ? 'calc(100vw - 1.5rem)' : 'calc(380px + 3rem)';

    function validateKey() {
        entry.keyErr = '';
        if (!isKeyUnique(entry.key)) {
            entry.keyErr = t.keyUnique;
        }
    }

</script>

<div class="entry">
    <div class="row1" style:gap={viewModePhone ? '0' : '2.5rem'}>
        <Input
                bind:value={entry.key}
                bind:error={entry.keyErr}
                autocomplete="off"
                placeholder={t.key}
                width={inputWidth}
                on:input={validateKey}
        >
            {t.key.toUpperCase()}
        </Input>

        <div
                role="button"
                tabindex="0"
                class="delete"
                on:click={() => removeKey(entry.key)}
                on:keypress={() => removeKey(entry.key)}
        >
            <IconStop color="var(--col-err)" width="1.5rem" />
        </div>
    </div>
    <Input
            bind:value={entry.value}
            bind:error={entry.valueErr}
            autocomplete="off"
            placeholder={entry.key}
            width={inputWidthValue}
    >
        {entry.key}
    </Input>
</div>

<style>
    .delete {
        margin-top: -.5rem;
        margin-left: .15rem;
        cursor: pointer;
    }

    .entry {
        margin: 1rem 0;
    }

    .row1 {
        display: flex;
        align-items: center;
    }
</style>
