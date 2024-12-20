<script>
    import { run } from 'svelte/legacy';

    import IconClipboard from "$lib/icons/IconClipboard.svelte";
    import IconEye from "$lib/icons/IconEye.svelte";
    import IconEyeSlash from "$lib/icons/IconEyeSlash.svelte";
    import {onMount} from "svelte";


    /**
     * @typedef {Object} Props
     * @property {string} [value]
     * @property {number} [rows]
     * @property {number} [cols]
     * @property {string} [name]
     * @property {boolean} [show]
     * @property {string} [width]
     */

    /** @type {Props & { [key: string]: any }} */
    let {
        value = '',
        rows = 10,
        cols = 60,
        name = 'default',
        show = $bindable(false),
        width = '40rem',
        ...rest
    } = $props();

    let hidden = $state('');
    let text = $state('');

    run(() => {
        if (show) {
            text = value;
        } else {
            text = hidden;
        }
    });

    onMount(() => {
        for (let i = 0; i < value.length; i++) {
            hidden = hidden + '*';
        }
        text = hidden;
    });

    function copyToClip() {
        navigator.clipboard.writeText(value);
    }

    function toggle() {
        show = !show;
    }

</script>

<div style:width={`${width}`}>
    <div class="iconsOuter">
        <div class="iconsInner">
            <div role="button" tabindex="0" class="show" onclick={toggle} onkeypress={toggle}>
                {#if show}
                    <IconEye width={22}/>
                {:else}
                    <IconEyeSlash width={22}/>
                {/if}
            </div>

            <div role="button" tabindex="0" onclick={copyToClip} onkeypress={copyToClip}>
                <IconClipboard/>
            </div>
        </div>
    </div>

    <textarea
            style:width={`${width}`}
            style:padding-right="2.75rem"
            disabled
            bind:value={text}
            {name}
            {rows}
            {cols}
            {...rest}
    ></textarea>
</div>

<style>
    .iconsOuter {
        position: relative;
    }

    .iconsInner {
        position: absolute;
        top: 2px;
        right: 1px;
        cursor: pointer;
        opacity: 0.85;
    }

    textarea {
        resize: none;
        border: 1px solid var(--col-inact);
        border-radius: 3px;
        outline: none;
    }

    textarea:focus {
        resize: none;
        border: 1px solid var(--col-acnt);
    }

    .show {
        position: absolute;
        top: -1px;
        right: 20px;
        cursor: pointer;
    }
</style>
