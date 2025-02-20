<script lang="ts">
    import type {ThemeCss} from "$api/types/themes.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import HSLInputRange from "$lib5/admin/clients/branding/HSLInputRange.svelte";
    import {PATTERN_CSS_VALUE_LOOSE} from "$utils/patterns.ts";
    import Input from "$lib5/form/Input.svelte";

    let {
        values = $bindable(),
        inputWidth,
    }: {
        values: ThemeCss,
        inputWidth: string,
    } = $props();

    let ta = useI18nAdmin();

</script>

<div class="container">
    <div>
        <p>HSL Values</p>
        <div class="hsl">
            <div>
                <HSLInputRange
                        label="--text"
                        bind:h={values.text[0]}
                        bind:s={values.text[1]}
                        bind:l={values.text[2]}
                />
                <HSLInputRange
                        label="--text-high"
                        bind:h={values.text_high[0]}
                        bind:s={values.text_high[1]}
                        bind:l={values.text_high[2]}
                />
                <HSLInputRange
                        label="--bg"
                        bind:h={values.bg[0]}
                        bind:s={values.bg[1]}
                        bind:l={values.bg[2]}
                />
                <HSLInputRange
                        label="--bg-high"
                        bind:h={values.bg_high[0]}
                        bind:s={values.bg_high[1]}
                        bind:l={values.bg_high[2]}
                />
            </div>

            <div>
                <HSLInputRange
                        label="--action"
                        bind:h={values.action[0]}
                        bind:s={values.action[1]}
                        bind:l={values.action[2]}
                />
                <HSLInputRange
                        label="--accent"
                        bind:h={values.accent[0]}
                        bind:s={values.accent[1]}
                        bind:l={values.accent[2]}
                />
                <HSLInputRange
                        label="--error"
                        bind:h={values.error[0]}
                        bind:s={values.error[1]}
                        bind:l={values.error[2]}
                />
            </div>
        </div>
    </div>

    <div>
        <p>CSS FULL</p>
        <Input
                label="btn-text"
                placeholder="btn-text"
                errMsg={ta.validation.css}
                width={inputWidth}
                bind:value={values.btn_text}
                required
                pattern={PATTERN_CSS_VALUE_LOOSE}
        />
        <Input
                label="theme-sun"
                placeholder="theme-sun"
                errMsg={ta.validation.css}
                width={inputWidth}
                bind:value={values.theme_sun}
                required
                pattern={PATTERN_CSS_VALUE_LOOSE}
        />
        <Input
                label="theme-moon"
                placeholder="theme-moon"
                errMsg={ta.validation.css}
                width={inputWidth}
                bind:value={values.theme_moon}
                required
                pattern={PATTERN_CSS_VALUE_LOOSE}
        />
    </div>
</div>

<style>
    .container {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
        /*border: 1px solid red;*/
    }

    .hsl {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
    }
</style>
