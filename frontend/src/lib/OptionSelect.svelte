<script>
    export let value;
    export let options = [];
    export let width = '120px';
</script>

<select bind:value style:width>
    {#each options as o}
        <option class="opt" value={o}>
            {o}
        </option>
    {/each}
</select>

<style>
    select {
        padding: 3px;
        color: var(--col-text);
        font-size: 1rem;
        cursor: pointer;
        border-radius: 3px;
        border: none;
        outline: none;
        background: white;
    }

    select:hover {
        color: var(--col-acnta);
    }

    .opt {
        color: var(--col-text);
        cursor: pointer;
    }
</style>
