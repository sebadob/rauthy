<script>
    import IconStop from "../icons/IconStop.svelte";

    export let label;
    export let onDelete = () => {
    };
</script>

<div class="label font-label">
    {label}
    <div
            role="button"
            tabindex="0"
            class="icon"
            on:click={() => onDelete(label)}
            on:keypress={() => onDelete(label)}
    >
        <IconStop width="14"/>
    </div>
</div>

<style>
    .label {
        position: relative;
        margin: 0 3px;
        padding: 5px 17px 5px 7px;
        border: 1px solid var(--col-acnt);
        border-radius: 6px;
        background: var(--col-acnta);
        color: white;
    }

    .icon {
        position: absolute;
        top: 0;
        right: 1px;
        cursor: pointer;
    }
</style>
