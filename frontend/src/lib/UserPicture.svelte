<script lang="ts">
    import IconUpload from "$icons/IconUpload.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {genKey} from "$utils/helpers.ts";
    import {buildHeaders} from "$api/fetch.ts";
    import type {ErrorResponse} from "$api/types/error.ts";

    let {
        userId,
        fallbackCharacters,
        // image_id will be appended to the path if given, and
        // its existence will be checked in the upload response
        pictureId = $bindable(),
        size = 'medium',
        uploadUri,
    }: {
        userId: string;
        fallbackCharacters: string;
        pictureId: undefined | string;
        size?: 'small' | 'medium' | 'large';
        uploadUri?: string;
    } = $props();

    const id = genKey();
    // const accept = 'image/png, image/jpeg, image/webp';
    const accept = 'image/png, image/jpeg';

    let t = useI18n();

    let err = $state('');

    let showUpload = $state(false);
    let isUploading = $state(false);
    let files: undefined | FileList = $state();

    let color = $derived(calculateColor());
    let width = $derived.by(() => {
        switch (size) {
            case 'small':
                return '1.5rem';
            case 'medium':
                return '3rem';
            default:
                return '12rem';
        }
    });

    $effect(() => {
        if (err) {
            console.error(err);
        }
    });

    $effect(() => {
        if (files && files.length > 0) {
            upload(files);
        }
    });

    function calculateColor() {
        let hue = stringToHue(fallbackCharacters);
        return `hsl(${hue}, 50%, 50%)`;
    }

    function ondrop(ev: DragEvent) {
        files = ev.dataTransfer?.files;
    }

    function onerror(ev: Event & { currentTarget: EventTarget & Element }) {
        if (ev.type === 'error') {
            err = 'Input Error';
        }
    }

    function onmouseenter() {
        showUpload = true;
    }

    function onmouseleave() {
        showUpload = false;
    }

    /// Converts the given string to a random hue.
    /// Will always return the same result for the same input.
    /// Ignores any characters after the first 5 for efficiency.
    function stringToHue(s: string) {
        let hues = [1, 255, 3, 13, 19];

        for (let i = 0; i < 5; i++) {
            let code = s.charCodeAt(i);
            if (code) {
                hues[i] = code;
            } else {
                break;
            }
        }

        return ((hues[0] * (64 + hues[1]) + 4 * hues[0]) + (hues[2] * hues[3] - hues[4])) % 360;
    }

    async function upload(list: FileList) {
        isUploading = true;

        for (let file of list) {
            if (!accept.includes(file.type)) {
                err = 'Invalid File Format, allowed: ' + accept;
                break;
            }
            if (file.size > 2 * 1024 * 1024) {
                err = 'max size 2MB';
                break;
            }

            let fd = new FormData();
            fd.append(file.name, file)
            let res = await fetch(`/auth/v1/users/${userId}/picture`, {
                method: 'PUT',
                headers: buildHeaders('PUT', 'form'),
                body: fd,
            });

            if (res) {
                if (res.ok) {
                    pictureId = await res.text();
                } else {
                    let error: ErrorResponse = await res.json();
                    err = error.message || 'Upload Error';
                }
            }
        }

        files = undefined;
        isUploading = false;
    }

</script>

{#snippet avatar()}
    {#if err || !pictureId}
        <span class={`absolute font-mono font-${size} noselect`}>
            {fallbackCharacters.toUpperCase()}
        </span>
    {:else}
        <img
                src={`/auth/v1/users/${userId}/pictures/${pictureId}`}
                loading="lazy"
                class="absolute"
                aria-label="Avatar"
                alt=""
                width="100%"
                {onerror}
        />
    {/if}
{/snippet}

{#if uploadUri}
    <form
            class="avatar"
            aria-dropeffect="move"
            aria-label="Upload"
            style:background-color={color}
            style:width
            {onmouseenter}
            {onmouseleave}
    >
        <label
                for={id}
                aria-controls={id}
                aria-disabled={isUploading}
                style:width
                data-show={!isUploading && showUpload}
        >
            <IconUpload {width}/>
        </label>
        <input
                {id}
                type="file"
                disabled={isUploading}
                aria-disabled={isUploading}
                aria-hidden="true"
                {accept}
                bind:files
        />

        {@render avatar()}
    </form>
{:else}
    <span
            class="avatar"
            style:background-color={color}
            style:width
            style:height={width}
    >
        {@render avatar()}
    </span>
{/if}

<style>
    input[type="file"]::file-selector-button {
        display: none;
    }

    input[aria-hidden="true"] {
        display: none;
    }

    label {
        position: absolute;
        opacity: 0;
        aspect-ratio: 1;
        background: hsla(var(--bg) / .8);
        cursor: pointer;
        z-index: 1;
        transition: all 150ms;
    }

    label[aria-disabled="true"] {
        cursor: default;
    }

    label[data-show="true"] {
        opacity: 1;
    }

    label:hover {
        color: hsl(var(--action));
    }

    .avatar {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        aspect-ratio: 1;
        border-radius: 33%;
        overflow: clip;
        cursor: default;
    }

    .font-small, .font-medium, .font-large {
        position: relative;
        color: hsl(var(--bg));
        font-weight: bold;
        text-align: center;
    }

    .font-small {
        font-size: 1.15rem;
    }

    .font-medium {
        top: -.1rem;
        font-size: 2.2rem;
    }

    .font-large {
        top: -.2rem;
        font-size: 9rem;
    }
</style>