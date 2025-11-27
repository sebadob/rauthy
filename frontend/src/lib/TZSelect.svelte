<script lang="ts">
    import Options from '$lib/Options.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { onMount } from 'svelte';
    import { fetchTimezones } from '$utils/helpers';

    let {
        value = $bindable('UTC'),
    }: {
        value: string | undefined | null;
    } = $props();

    let t = useI18n();
    let options: string[] = $state(['UTC']);

    onMount(() => {
        if (value && value !== 'UTC') {
            options.push(value);
        }
        fetchTimezones().then(tz => (options = tz));
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
        padding-top: 0.5rem;
    }

    .label {
        margin-top: -0.25rem;
        margin-left: 0.5rem;
        color: hsl(var(--text) / 0.8);
        font-size: 0.9rem;
    }

    .opts {
        border-bottom: 1px solid hsl(var(--bg-high));
    }
</style>
