<script>
    import IconClipboard from "$lib/icons/IconClipboard.svelte";
    import IconEye from "$lib/icons/IconEye.svelte";
    import IconEyeSlash from "$lib/icons/IconEyeSlash.svelte";
    import {onMount} from "svelte";

    export let value = '';
    export let rows = 10;
    export let cols = 60;
    export let name = 'default';

    export let show = false;
    export let width = '40rem';

    let hidden = '';
    let text = '';

    $: if (show) {
        text = value;
    } else {
        text = hidden;
    }

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
            <div role="button" tabindex="0" class="show" on:click={toggle} on:keypress={toggle}>
                {#if show}
                    <IconEye width={22}/>
                {:else}
                    <IconEyeSlash width={22}/>
                {/if}
            </div>

            <div role="button" tabindex="0" on:click={copyToClip} on:keypress={copyToClip}>
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
            {...$$restProps}
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
