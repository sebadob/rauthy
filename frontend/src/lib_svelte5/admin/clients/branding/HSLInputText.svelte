<script lang="ts">
    import {PATTERN_CSS_VALUE_LOOSE} from "$utils/patterns.ts";
    import Input from "$lib5/form/Input.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

    let {
        label,
        value = $bindable(),
        inputWidth,
    }: {
        label: string,
        value: string,
        inputWidth: string,
    } = $props();

    let ta = useI18nAdmin();

    let ref: undefined | HTMLInputElement = $state();

</script>

<div class="container" style:--color={value}>
    <Input
            {label}
            placeholder={label}
            errMsg={ta.validation.css}
            width={inputWidth}
            bind:value
            required
            pattern={PATTERN_CSS_VALUE_LOOSE}
    />
    <div role="none" class="mask" onclick={() => ref?.click()}>
        <div class="colorWrapper">
            <div class="color"></div>
        </div>
        <input bind:this={ref} type="color" bind:value/>
    </div>
</div>

<style>
    input[type="color"] {
        margin-left: -.6rem;
        margin-top: -.4rem;
        height: 6rem;
        width: 4.5rem;
        cursor: pointer;
    }

    .container {
        width: 16rem;
        margin: .5rem 0;
        display: flex;
        border: 1px solid var(--color);
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .color {
        position: absolute;
        top: 0;
        left: 0;
        height: 4.8rem;
        width: 3rem;
        background: var(--color);
        cursor: pointer;
    }

    .colorWrapper {
        position: relative;
    }

    .mask {
        height: 4.8rem;
        width: 3rem;
        overflow: clip;
    }
</style>
