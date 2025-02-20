<script lang="ts">
    import {genKey} from "$utils/helpers.ts";

    let {
        name,
        value = $bindable(),
        label = '',
        disabled = false,
        min,
        max,
        step = 1,
        widthRange = '15rem',
        bgMode,
        hue,
        sat,
        lum,
    }: {
        name?: string,
        value: number,
        label: string,
        disabled?: boolean | null | undefined,
        min: number,
        max: number,
        step?: number,
        widthRange?: string,
        bgMode?: 'generic' | 'hue' | 'sat' | 'lum',
        hue?: number,
        sat?: number,
        lum?: number,
    } = $props();

    const id = genKey();

    let styleBg = $derived.by(() => {
        if (hue && sat && lum) {
            if (bgMode === 'hue') {
                return `linear-gradient(
                    to right,
                    hsl(0 ${sat} ${lum}) 0%,
                    hsl(60 ${sat} ${lum}) 17%,
                    hsl(120 ${sat} ${lum}) 33%,
                    hsl(180 ${sat} ${lum}) 50%,
                    hsl(240 ${sat} ${lum}) 67%,
                    hsl(300 ${sat} ${lum}) 83%,
                    hsl(3600 ${sat} ${lum}) 100%
                )`;
            }
            if (bgMode === 'sat') {
                return `linear-gradient(to right, hsl(${hue} 0 ${lum}), hsl(${hue} 100 ${lum}))`;
            }
            if (bgMode === 'lum') {
                return `linear-gradient(to right, hsl(${hue} ${sat} 0), hsl(${hue} ${sat} 100))`;
            }
        }

        return 'hsl(var(--text) / .5)';
    });
</script>

<div>
    <div class="flex">
        <input
                type="range"
                {id}
                {name}
                style:width={widthRange}
                style:--bg-slider={styleBg}
                title={label}
                aria-label={label}
                bind:value

                {disabled}
                aria-disabled={disabled}

                {min}
                {max}
                {step}
        />
        <div class="value font-mono">{value}</div>
    </div>
    <div class="label">
        <label for={id} class="font-label noselect">
            {label}
        </label>
    </div>
</div>

<style>
    input {
        border: none;
        height: 1rem;
        cursor: pointer;
        appearance: none;
    }

    input[type=range]::-webkit-slider-runnable-track,
    input[type=range]::-moz-range-track {
        background: var(--bg-slider);
        /*background: hsl(var(--text) / .5);*/
        height: 5px;
        border-radius: var(--border-radius);
    }

    input[type=range]::-webkit-slider-thumb,
    input[type=range]::-moz-range-thumb {
        height: 1.2rem;
        width: .66rem;
        background: hsl(var(--text-high));
        border: none;
        border-radius: var(--border-radius);
    }

    input[aria-disabled="true"] {
        cursor: not-allowed;
    }

    input[aria-disabled="true"]::-webkit-slider-runnable-track,
    input[aria-disabled="true"]::-moz-range-track {
        background: hsl(var(--accent));
    }

    label {
        line-height: 1.1rem;
        font-size: .9rem;
        color: hsla(var(--text) / .8);
        flex-wrap: wrap;
        width: 100%;
        padding: .5rem;
    }

    .label {
        margin-top: -.66rem;
    }

    .value {
        color: hsl(var(--text-high));
    }
</style>
