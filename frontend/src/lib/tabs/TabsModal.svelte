<script lang="ts">
    import Tabs from '$lib5/tabs/Tabs.svelte';
    import { untrack } from 'svelte';

    let {
        tabs,
        selected = $bindable(''),
        center = false,
        width = 'inherit',
    }: {
        tabs: string[];
        selected: string;
        borderRadius?: string;
        center?: boolean;
        width?: string;
    } = $props();

    untrack(() => {
        if (tabs.length > 0 && selected === '') {
            selected = tabs[0];
        }
    });
</script>

<div class="wrapper">
    <Tabs {tabs} bind:selected borderRadius="none" {center} {width} />
</div>

<style>
    .wrapper {
        margin: -1rem -1rem 1rem -1rem;
    }
</style>
