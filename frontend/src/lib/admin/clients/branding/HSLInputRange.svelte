<script lang="ts">
    import InputRange from '#lib/form/InputRange.svelte';
    import InputColor from '#lib/form/InputColor.svelte';

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

    const widthRange = '100%';

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

<div class="outer" role="group" aria-label={label}>
    <div class="container" style:border-color={hsl}>
        <div>
            <h4>{label}</h4>
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
        </div>
        <div class="color">
            <InputColor {label} bind:value={() => hex, v => setHex(v)} />
        </div>
    </div>
</div>

<style>
    .outer {
        margin-bottom: 0.75rem;
    }

    .container {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 2.5rem;
        align-items: stretch;
        border: 1px solid;
        border-radius: var(--border-radius);
        background: var(--ui-container);
    }

    .container > div:first-child {
        min-width: 0;
        padding: 0.75rem;
    }

    h4 {
        margin: 0 0 0.5rem;
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
    }

    .color {
        width: 2.5rem;
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        overflow: clip;
    }

    .values {
        display: grid;
        gap: 0.25rem;
    }
</style>
