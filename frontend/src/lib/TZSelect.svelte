<script lang="ts">
    import Options from '$lib/Options.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { onMount } from 'svelte';
    import { fetchTimezones } from '$utils/helpers';

    let {
        value = $bindable('UTC'),
    }: {
        value?: string;
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
    <div class="label">
        {t.common.timezone}
    </div>
    <div class="opts">
        <Options ariaLabel={t.common.timezone} {options} bind:value withSearch />
    </div>
</div>

<style>
    .label {
        margin-bottom: -0.3rem;
        padding-left: 0.1rem;
        padding-top: 0.15rem;
        color: hsl(var(--text));
        font-size: 0.9rem;
    }

    .opts {
        margin: 0.5rem 0;
    }
</style>
