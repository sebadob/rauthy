<script lang="ts">
    import { genKey } from '$utils/helpers';
    import InputRange from '$lib5/form/InputRange.svelte';
    import InputColor from '$lib5/form/InputColor.svelte';

    let {
        label,
        h = $bindable(),
        s = $bindable(),
        l = $bindable(),
    }: {
        label: string;
        h: number;
        s: number;
        l: number;
    } = $props();

    const id = genKey();
    const widthRange = '15rem';

    let hsl = $derived(`hsl(${h} ${s}% ${l}%)`);
    let hex = $derived(hslToHex(h, s, l));

    function hslToHex(h: number, s: number, l: number) {
        const a = (s / 100) * Math.min(l / 100, 1 - l / 100);
        const f = (n: number) => {
            const k = (n + h / 30) % 12;
            const c = l / 100 - a * Math.max(-1, Math.min(k - 3, 9 - k, 1));
            return Math.round(c * 255)
                .toString(16)
                .padStart(2, '0');
        };
        return `#${f(0)}${f(8)}${f(4)}`;
    }

    function setHex(value: string) {
        const [r, g, b] = [1, 3, 5].map(i => parseInt(value.slice(i, i + 2), 16) / 255);
        const max = Math.max(r, g, b);
        const min = Math.min(r, g, b);
        const d = max - min;
        const lum = (max + min) / 2;

        let hue = 0;
        if (d !== 0) {
            if (max === r) hue = ((g - b) / d) % 6;
            else if (max === g) hue = (b - r) / d + 2;
            else hue = (r - g) / d + 4;
        }

        h = (Math.round(hue * 60) + 360) % 360;
        s = d === 0 ? 0 : Math.round((d / (1 - Math.abs(2 * lum - 1))) * 100);
        l = Math.round(lum * 100);
    }
</script>

<div class="outer">
    <div class="container" style:border-color={hsl}>
        <div>
            <h5>
                <label for={id} class="font-label">
                    {label}
                </label>
            </h5>
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
        <div class="color">
            <InputColor {label} bind:value={() => hex, v => setHex(v)} />
        </div>
    </div>
</div>

<style>
    h5 {
        margin: 0 0 0.25rem 0;
    }

    label {
        color: hsl(var(--text-high));
        margin-left: 0.5rem;
    }

    .color {
        height: 8.5rem;
        width: 3rem;
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        overflow: clip;
    }

    .container {
        height: 8.5rem;
        display: flex;
        gap: 0.5rem;
        background: hsla(var(--bg-high) / 0.2);
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
    }

    .outer {
        margin: 0.5rem 0;
    }

    .values {
        width: 17rem;
        line-height: 1rem;
    }
</style>
