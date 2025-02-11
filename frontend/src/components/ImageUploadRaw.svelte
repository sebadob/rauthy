<script lang="ts">
    import IconUpload from "$lib/icons/IconUpload.svelte";

    let {
        text = 'Upload Logo',
    }: {
        text: string,
    } = $props();

    let inputRef: undefined | HTMLInputElement = $state();
    let file: undefined | File = $state();

    async function onFileSelected(ev: Event) {
        file = ev.target.files[0];
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
        color: hsl(var(--action));
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
