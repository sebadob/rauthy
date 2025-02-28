<script lang="ts">
    import {genKey} from "$utils/helpers.ts";
    import Loading from "$lib5/Loading.svelte";
    import IconUpload from "$icons/IconUpload.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {uploadFile} from "$api/fetch.ts";

    let {
        // typ,
        method = 'PUT',
        accept = ['.image/jpg', 'image/jpeg', 'image/png', 'image/svg'],
        url,
        disabled,
        buttonOnly = false,
        buttonSize = '1.5rem',
        fileName,
        multiple = false,
        onSuccess,
        width = '17rem',
        height = '2.5rem',
    }: {
        // typ: 'avatar' | 'fileType',
        method?: 'POST' | 'PUT',
        accept?: string[];
        url: string;
        disabled?: boolean;
        buttonOnly?: boolean;
        buttonSize?: string;
        fileName?: string;
        multiple?: boolean;
        onSuccess?: () => void;
        width?: string;
        height?: string;
    } = $props();

    const id = genKey();

    let ta = useI18nAdmin();

    let isUploading = $state(false);

    let files: undefined | FileList = $state();
    let filesToUpload = $state(0);
    let filesFinished = $state(0);

    $effect(() => {
        if (!isUploading && files && files.length > 0) {
            upload(files);
        }
    });

    function ondrop(ev: DragEvent) {
        files = ev.dataTransfer?.files;
    }

    async function upload(list: FileList) {
        filesToUpload = list.length;
        filesFinished = 0;
        isUploading = true;

        for (let file of list) {
            if (accept && !accept.includes(file.type)) {
                filesFinished += 1;
                continue;
            }

            let res = await uploadFile(method, url, file, fileName || file.name);
            if (res.error) {
                console.error(res.error);
            } else {
                onSuccess?.();
            }

            filesFinished += 1;
        }

        files = undefined;
        isUploading = false;
    }

</script>

<form
        aria-dropeffect="move"
        aria-label="Upload"
        class="flex"
        data-nopad={buttonOnly}
        style:width
        style:height
        {ondrop}
>
    {#if isUploading}
        <div class="flex space-between loading">
            <span class="flex gap-05">
                <span>
                    {ta.common.loading}:
                </span>
                <span>{filesFinished}</span>
                <span>/</span>
                <span>{filesToUpload}</span>
            </span>
            <Loading/>
        </div>
    {:else}
        <label for={id} aria-disabled={disabled}>
            <IconUpload width={buttonSize}/>
        </label>
        <input
                type="file"
                {id}
                aria-hidden={buttonOnly}
                accept={accept && accept.join(', ')}
                bind:files
                aria-disabled={disabled}
                {disabled}
                {multiple}
        />
    {/if}
</form>

<style>
    input[type="file"] {
        background: transparent;
        border: none;
        color: hsla(var(--text) / .5);
        cursor: pointer;
    }

    input[type="file"]::file-selector-button {
        display: none;
    }

    input[aria-hidden="true"] {
        display: none;
    }

    input[type="file"]:disabled {
        cursor: not-allowed;
        color: hsl(var(--bg-high))
    }

    form {
        padding-left: .5rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        transition: background 150ms;
    }

    form[data-nopad="true"] {
        padding: 0;
        aspect-ratio: 1;
    }

    form[data-nopad="true"] label {
        flex: 1;
        text-align: center;
    }

    form:hover {
        background: hsl(var(--bg-high));
    }

    label {
        position: relative;
        bottom: -.2rem;
        cursor: pointer;
        transition: color 150ms;
    }


    label:hover {
        color: hsl(var(--action));
    }

    label[aria-disabled="true"],
    label[aria-disabled="true"]:hover {
        color: hsla(var(--text) / .5);
    }

    .loading {
        flex: 1;
    }
</style>
