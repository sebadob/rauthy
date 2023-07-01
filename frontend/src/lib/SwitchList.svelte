<script>
    import Switch from "./Switch.svelte";

    export let options = [
        {
            label: 'password',
            value: false,
        },
        {
            label: 'refresh_token',
            value: false,
        },
    ];

</script>

<div class="list">
    {#each options as opt}
        <div class="entry">
            <div class="label">
                {opt.label}
            </div>
            <Switch bind:selected={opt.value}/>
        </div>
    {/each}
</div>

<style>
    .entry {
        display: flex;
        justify-content: flex-end;
        align-items: center;
    }

    .label {
        flex: 1;
        margin: 5px 10px 2px 0;
    }

    .list {
        display: flex;
        flex-direction: column;
        margin-left: 1px;
    }
</style>
