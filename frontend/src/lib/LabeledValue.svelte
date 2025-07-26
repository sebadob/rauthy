<script lang="ts">
    import type {Snippet} from "svelte";
    import IconClipboard from "$icons/IconClipboard.svelte";
    import Button from "$lib/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import Tooltip from "$lib/Tooltip.svelte";

    let {
        label,
        title,
        mono,
        copyToClip,
        button,
        children,
    }: {
        label?: string,
        title?: string,
        mono?: boolean,
        copyToClip?: string,
        button?: Snippet,
        children: Snippet,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let text = $state(t.common.copyToClip);

    function copy(value: string) {
        navigator.clipboard.writeText(value);
        text = ta.common.copiedToClip;
        setTimeout(() => {
            text = t.common.copyToClip;
        }, 3000);
    }
</script>

<div class="container">
    {#if label}
        <div class="label font-label">
            {label}
        </div>
    {/if}
    <div class="flex gap-05">
        <div title={title || label || ''} class={mono ? 'font-mono' : ''}>
            {@render children()}
        </div>
        {#if button}
            <div class="button">
                {@render button()}
            </div>
        {/if}
        {#if copyToClip}
            <div class="button">
                <Button invisible onclick={() => copy(copyToClip)}>
                    <Tooltip {text} xOffset={-150} yOffset={20}>
                        <IconClipboard width="1.25rem"/>
                    </Tooltip>
                </Button>
            </div>
        {/if}
    </div>
</div>

<style>
    .button {
        margin-top: -.15rem;
        margin-bottom: -.5rem;
    }

    .container {
        margin: .35rem 0;
    }

    .label {
        margin-bottom: -.45rem;
        font-size: .85rem;
        color: hsla(var(--text) / .7);
    }
</style>
