<script lang="ts">
    import IconUpload from "$icons/IconUpload.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {genKey, getCsrfToken} from "$utils/helpers";
    import type {ErrorResponse} from "$api/types/error.ts";
    import IconStop from "$icons/IconStop.svelte";
    import Button from "$lib/button/Button.svelte";
    import {fetchDelete} from "$api/fetch";
    import {usePictureConfig} from "$state/picture_config.svelte";

    let {
        userId,
        fallbackCharacters,
        // image_id will be appended to the path if given, and
        // its existence will be checked in the upload response
        pictureId = $bindable(),
        size = 'medium',
        disableUpload,
    }: {
        userId?: string;
        fallbackCharacters: string | undefined;
        pictureId?: string;
        size?: 'small' | 'medium' | 'large';
        disableUpload?: boolean,
    } = $props();

    const id = genKey();
    const accept = 'image/png, image/jpeg, image/webp, image/svg+xml';

    let t = useI18n();

    let refDel: undefined | HTMLButtonElement = $state();

    let config = $derived(usePictureConfig().get());

    let err = $state('');
    let errFileSize = $state('');
    let timer: undefined | number;

    let showUpload = $state(false);
    let isUploading = $state(false);
    let files: undefined | FileList = $state();

    let color = $derived(calculateColor());
    let width = $derived.by(() => {
        switch (size) {
            case 'small':
                return '2.25rem';
            case 'medium':
                return '3rem';
            default:
                // max size of the picture saved in the background
                return '192px';
        }
    });

    $effect(() => {
        if (err) {
            console.error(err);
        }
    });

    $effect(() => {
        if (errFileSize) {
            if (timer) {
                clearTimeout(timer);
            }
            timer = setTimeout(() => {
                errFileSize = '';
            }, 5000);
        }
    });

    $effect(() => {
        if (files && files.length > 0) {
            upload(files);
        }
    });

    function calculateColor() {
        let hue = 0;
        if (fallbackCharacters) {
            hue = stringToHue(fallbackCharacters);
        }
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

    async function deletePicture() {
        err = '';
        errFileSize = '';

        if (!pictureId) {
            return;
        }

        let res = await fetchDelete(`/auth/v1/users/${userId}/picture/${pictureId}`);
        if (res.error) {
            err = res.error.message;
        } else {
            pictureId = undefined;
        }
    }

    async function upload(list: FileList) {
        err = '';
        errFileSize = '';

        if (!config) {
            return;
        }

        isUploading = true;

        let url = `/auth/v1/users/${userId}/picture`;

        for (let file of list) {
            if (!accept.includes(file.type)) {
                err = 'Invalid File Format, allowed: ' + accept + ', found: ' + file.type;
                break;
            }
            if (file.size > config.content_len_limit) {
                errFileSize = `${t.common.maxFileSize}: ${config.content_len_limit / 1024 / 1024} MB`;
                break;
            }

            let fd = new FormData();
            fd.append(file.name, file)
            let res = await fetch(url, {
                method: 'PUT',
                headers: {
                    'x-csrf-token': getCsrfToken(),
                },
                body: fd,
            });

            if (res.ok) {
                pictureId = await res.text();
                setTimeout(() => {
                    refDel?.focus();
                }, 100);
            } else {
                let error: ErrorResponse = await res.json();
                if (res.status === 406) {
                    err = 'File too big: ' + error.message;
                } else {
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
            {fallbackCharacters ? fallbackCharacters.toUpperCase() : ''}
        </span>
    {:else}
        <img
                src={`/auth/v1/users/${userId}/picture/${pictureId}`}
                loading="lazy"
                class="absolute"
                aria-label="Avatar"
                alt=""
                width="100%"
                {onerror}
        />
    {/if}
{/snippet}

<div class="container">
    {#if config?.upload_allowed && !disableUpload}
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
                    tabindex="0"
                    {id}
                    type="file"
                    disabled={isUploading}
                    aria-label="Upload Picture"
                    aria-disabled={isUploading}
                    onfocusin={() => showUpload = true}
                    onfocusout={() => showUpload = false}
                    {accept}
                    bind:files
            />

            {#if errFileSize}
                <div class="errLimit" style:width>
                    {errFileSize}
                </div>
            {/if}

            {@render avatar()}
        </form>

        {#if size === 'large' && pictureId}
            <div class="relative">
                <div class="delete">
                    <Button bind:ref={refDel} invisible onclick={deletePicture}>
                        <div title={t.common.delete}>
                            <IconStop/>
                        </div>
                    </Button>
                </div>
            </div>
        {/if}
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
</div>

<style>
    img {
        background: hsl(var(--bg));
    }

    input {
        margin: 0;
        padding: 0;
        width: 0;
        height: 0;
        overflow: hidden;
    }

    input[type="file"]::file-selector-button {
        display: none;
    }

    label {
        position: absolute;
        opacity: 0;
        aspect-ratio: 1;
        background: hsla(var(--bg) / .8);
        cursor: pointer;
        z-index: 2;
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
        border-radius: 30%;
        overflow: clip;
        cursor: default;
    }

    .container {
        display: flex;
    }

    .delete {
        position: absolute;
        top: -.1rem;
        left: -1.25rem;
    }

    .errLimit {
        padding: 0 .25rem;
        background: hsl(var(--error));
        border-radius: var(--border-radius);
        font-weight: bold;
        text-wrap: wrap;
        z-index: 1;
    }

    .font-small, .font-medium, .font-large {
        position: relative;
        color: hsl(var(--bg));
        font-weight: bold;
        text-align: center;
    }

    .font-small {
        top: -.05rem;
        font-size: 1.5rem;
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
