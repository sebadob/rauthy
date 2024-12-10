<script>
    /**
     * @typedef {Object} Props
     * @property {any} value
     * @property {any} [options]
     * @property {string} [width]
     */

    /** @type {Props} */
    let { value = $bindable(), options = [], width = 'inherit' } = $props();
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
        background: transparent;
    }

    select:hover {
        color: var(--col-acnta);
    }

    .opt {
        color: var(--col-text);
        cursor: pointer;
    }
</style>
