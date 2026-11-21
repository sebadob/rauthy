<script lang="ts">
    import { genKey } from '#utils/helpers.ts';

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
        name?: string;
        value: number;
        label: string;
        disabled?: boolean | null | undefined;
        min: number;
        max: number;
        step?: number;
        widthRange?: string;
        bgMode?: 'generic' | 'hue' | 'sat' | 'lum';
        hue?: number;
        sat?: number;
        lum?: number;
    } = $props();

    const id = genKey();

    let styleBg = $derived.by(() => {
        if (hue !== undefined && sat !== undefined && lum !== undefined) {
            if (bgMode === 'hue') {
                return `linear-gradient(
                    to right,
                    hsl(0 ${sat}% ${lum}%) 0%,
                    hsl(60 ${sat}% ${lum}%) 17%,
                    hsl(120 ${sat}% ${lum}%) 33%,
                    hsl(180 ${sat}% ${lum}%) 50%,
                    hsl(240 ${sat}% ${lum}%) 67%,
                    hsl(300 ${sat}% ${lum}%) 83%,
                    hsl(360 ${sat}% ${lum}%) 100%
                )`;
            }
            if (bgMode === 'sat') {
                return `linear-gradient(to right, hsl(${hue} 0% ${lum}%), hsl(${hue} 100% ${lum}%))`;
            }
            if (bgMode === 'lum') {
                return `linear-gradient(to right, hsl(${hue} ${sat}% 0%), hsl(${hue} ${sat}% 100%))`;
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
        flex: 1;
        min-width: 0;
        border: 0;
        padding: 0;
        background: transparent;
        height: 1.25rem;
        cursor: pointer;
        appearance: none;
    }

    input::-webkit-slider-runnable-track {
        background: var(--bg-slider);
        height: 6px;
        border-radius: var(--border-radius);
    }

    input::-moz-range-track {
        background: var(--bg-slider);
        height: 6px;
        border-radius: var(--border-radius);
    }

    input::-webkit-slider-thumb {
        appearance: none;
        margin-top: -4px;
        width: 14px;
        height: 14px;
        background: hsl(var(--text-high));
        border: 2px solid hsl(var(--bg));
        border-radius: 50%;
    }

    input::-moz-range-thumb {
        width: 12px;
        height: 12px;
        background: hsl(var(--text-high));
        border: 2px solid hsl(var(--bg));
        border-radius: 50%;
    }

    input:focus-visible {
        outline: 2px solid hsl(var(--action));
        outline-offset: 2px;
    }

    input:disabled {
        cursor: not-allowed;
    }

    label {
        line-height: 1.1rem;
        font-size: 0.9rem;
        color: hsla(var(--text) / 0.8);
        flex-wrap: wrap;
        width: 100%;
        padding: 0;
    }

    .label {
        margin-top: -0.25rem;
    }

    .value {
        min-width: 2.5rem;
        padding-inline-start: 0.5rem;
        font-size: 0.8125rem;
        color: hsl(var(--text-high));
    }
</style>
