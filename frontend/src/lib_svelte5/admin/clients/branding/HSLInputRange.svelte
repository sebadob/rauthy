<script lang="ts">
    import {genKey} from "$utils/helpers.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import InputRange from "$lib5/form/InputRange.svelte";

    let {
        label,
        h = $bindable(),
        s = $bindable(),
        l = $bindable(),
    }: {
        label: string,
        h: number,
        s: number,
        l: number,
    } = $props();

    const id = genKey();
    const widthRange = "15rem";

    let hsl = $derived(`hsl(${h} ${s} ${l})`);
</script>

<div class="outer">
    <div class="container" style:border-color={hsl}>
        <div>
            <label for={id} class="font-label">
                {label}
            </label>
            <div {id} class="values">
                <InputRange
                        label="Hue"
                        bind:value={h}
                        min={0}
                        max={359}
                        {widthRange}
                        bgMode="hue"
                        hue={h}
                        sat={s}
                        lum={l}
                />
                <InputRange
                        label="Sat"
                        bind:value={s}
                        min={0}
                        max={100}
                        {widthRange}
                        bgMode="sat"
                        hue={h}
                        sat={s}
                        lum={l}
                />
                <InputRange
                        label="Lum"
                        bind:value={l}
                        min={0}
                        max={100}
                        {widthRange}
                        bgMode="lum"
                        hue={h}
                        sat={s}
                        lum={l}
                />
            </div>
        </div>
        <div class="color" style:background={hsl}></div>
    </div>
</div>

<style>
    label {
        color: hsl(var(--text-high));
        margin-left: .5rem;
    }

    .color {
        height: 8.5rem;
        width: 3rem;
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
    }

    .container {
        height: 8.5rem;
        display: flex;
        gap: .5rem;
        background: hsla(var(--bg-high) / .2);
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
    }

    .outer {
        margin: .5rem 0;
    }

    .values {
        width: 17rem;
        line-height: 1rem;
    }
</style>
