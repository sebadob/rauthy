<script>
    import IconUpload from "$lib/icons/IconUpload.svelte";

    export let text = 'UPLOAD LOGO';
    export let image;

    let inputRef;

    async function onFileSelected(e) {
        image = e.target.files[0];
    }

</script>

<div class="container">
    <div
            role="button"
            tabindex="0"
            class="btn"
            on:click={() => inputRef.click()}
            on:keypress={() => inputRef.click()}
    >
        <IconUpload width={22}/>
    </div>
    <div
            role="button"
            tabindex="0"
            class="text noselect font-label"
            on:click={() => inputRef.click()}
            on:keypress={() => inputRef.click()}
    >
        {text}
    </div>

    <input
            bind:this={inputRef}
            class="imageInput"
            type="file"
            accept=".jpg, .jpeg, .png, .svg"
            on:change={(e)=>onFileSelected(e)}
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
