<script>
    import IconUpload from "$lib/icons/IconUpload.svelte";

    /**
     * @typedef {Object} Props
     * @property {string} [text]
     * @property {any} image
     */

    /** @type {Props} */
    let { text = 'UPLOAD LOGO', image = $bindable() } = $props();

    let inputRef = $state();

    async function onFileSelected(e) {
        image = e.target.files[0];
    }

</script>

<div class="container">
    <div
            role="button"
            tabindex="0"
            class="btn"
            onclick={() => inputRef.click()}
            onkeypress={() => inputRef.click()}
    >
        <IconUpload width={22}/>
    </div>
    <div
            role="button"
            tabindex="0"
            class="text noselect font-label"
            onclick={() => inputRef.click()}
            onkeypress={() => inputRef.click()}
    >
        {text}
    </div>

    <input
            bind:this={inputRef}
            class="imageInput"
            type="file"
            accept=".jpg, .jpeg, .png, .svg"
            onchange={(e)=>onFileSelected(e)}
    >
</div>

<style>
    .btn, .text {
        color: var(--col-act2);
    }

    .btn {
        margin-right: 5px;
        cursor: pointer;
    }

    .container {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .imageInput {
        display: none;
    }

    .text {
        font-size: .9rem;
        cursor: pointer;
    }
</style>
