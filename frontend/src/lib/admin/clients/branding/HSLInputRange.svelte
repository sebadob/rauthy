<script lang="ts">
    import InputRange from '#lib/form/InputRange.svelte';
    import InputColor from '#lib/form/InputColor.svelte';

    let {
        label,
        group,
        h = $bindable(),
        s = $bindable(),
        l = $bindable(),
    }: {
        label: string;
        group: string;
        h: number;
        s: number;
        l: number;
    } = $props();

    const widthRange = '100%';

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

<div class="container" role="group" aria-label={label}>
    <details name={group}>
        <summary><h4 class="font-mono">{label}</h4></summary>
        <div class="values">
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
    </details>
    <div class="color">
        <InputColor {label} bind:value={() => hex, v => setHex(v)} />
    </div>
</div>

<style>
    .container {
        position: relative;
        min-width: 0;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-container);
    }

    summary {
        min-height: 2.75rem;
        padding: 0.625rem 3rem 0.625rem 0.75rem;
        color: var(--ui-muted);
        cursor: pointer;
        border-radius: var(--border-radius);
    }

    summary:hover {
        background: var(--ui-hover);
    }

    h4 {
        display: inline;
        margin: 0;
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
    }

    .color {
        position: absolute;
        inset-block-start: 0.5rem;
        inset-inline-end: 0.5rem;
        width: 2rem;
        height: 1.75rem;
    }

    .color :global(input) {
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
    }

    .color :global(input::-webkit-color-swatch) {
        border-radius: inherit;
    }

    .color :global(input::-moz-color-swatch) {
        border-radius: inherit;
    }

    .values {
        display: grid;
        gap: 0.25rem;
        padding: 0.75rem;
        border-top: 1px solid var(--ui-outline);
    }
</style>
