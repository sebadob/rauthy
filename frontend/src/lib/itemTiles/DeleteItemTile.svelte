<script>
    import IconStop from "$lib/icons/IconStop.svelte";

    let { label, onDelete = () => {
    } } = $props();
</script>

<div class="label font-label">
    {label}
    <div
            role="button"
            tabindex="0"
            class="icon"
            onclick={() => onDelete(label)}
            onkeypress={() => onDelete(label)}
    >
        <IconStop color="--var(--col-bg)" width={14}/>
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
