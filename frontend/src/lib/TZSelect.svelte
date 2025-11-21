<script lang="ts">
    import Options from "$lib/Options.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {onMount} from "svelte";
    import {fetchTimezones} from "$utils/helpers";

    let {
        value = $bindable('Etc/UTC'),
    }: {
        value: string | undefined,
    } = $props();

    let t = useI18n();
    let options: string[] = $state(['Etc/UTC'])

    onMount(() => {
        if (value && value !== 'Etc/UTC') {
            options.push(value);
        }
        fetchTimezones().then(tz => options = tz);
    });
</script>

<div class="tz">
    <div class="opts">
        <Options
                ariaLabel={t.common.timezone}
                {options}
                bind:value
                borderless
                withSearch
        />
    </div>
    <div class="label">
        {t.common.timezone}
    </div>
</div>

<style>
    .tz {
        padding-top: .5rem;
    }

    .tz > .label {
        margin-top: -.25rem;
        margin-left: .5rem;
        color: hsl(var(--text) / .8);
        font-size: .9rem;
    }

    .tz > .opts {
        border-bottom: 1px solid hsl(var(--bg-high));
    }
</style>
