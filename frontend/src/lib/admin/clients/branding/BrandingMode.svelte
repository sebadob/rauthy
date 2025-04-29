<script lang="ts">
    import type {ThemeCss} from "$api/types/themes.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import HSLInputRange from "$lib5/admin/clients/branding/HSLInputRange.svelte";
    import HSLInputText from "$lib5/admin/clients/branding/HSLInputText.svelte";

    let {
        values = $bindable(),
    }: {
        values: ThemeCss,
    } = $props();

    let ta = useI18nAdmin();

</script>

<div>
    <p>{ta.clients.branding.descHsl}</p>
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

    <div>
        <p>{@html ta.clients.branding.descFullCss}</p>
        <HSLInputText
                label="--btn-text"
                bind:value={values.btn_text}
        />
        <HSLInputText
                label="--theme-sun"
                bind:value={values.theme_sun}
        />
        <HSLInputText
                label="--theme-moon"
                bind:value={values.theme_moon}
        />
    </div>
</div>


<style>
    .hsl {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
    }
</style>
