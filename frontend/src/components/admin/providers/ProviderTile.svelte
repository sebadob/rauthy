<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import ProviderConfig from "$lib5/admin/providers/ProviderConfig.svelte";
    import ProviderDelete from "$lib5/admin/providers/ProviderDelete.svelte";

    let {provider = $bindable({}), onSave = $bindable()} = $props();

    let expandContainer = $state();

    const tabBarItems = [
        'Config',
        'Delete',
    ];
    let selected = $state(tabBarItems[0]);
    const tabBarDur = 200;
    const tabBarDly = tabBarDur / 2;

    function onDelete() {
        expandContainer = false;
        onSave();
    }

</script>

<ExpandContainer bind:show={expandContainer}>
    {#snippet header()}
        <div class="header">
            <Tooltip text="Provider ID">
                <div class="data font-mono">
                    {provider.id}
                </div>
            </Tooltip>

            <Tooltip text="Provider Name">
                <div class="data">
                    {provider.name}
                </div>
            </Tooltip>
        </div>
    {/snippet}

    {#snippet body()}
        <div>
            <TabBar labels={tabBarItems} bind:selected/>

            {#if selected === 'Config'}
                <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                    <ProviderConfig bind:provider bind:onSave/>
                </div>

            {:else if selected === 'Delete'}
                <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                    <ProviderDelete bind:provider bind:onSave/>
                </div>
            {/if}
        </div>
    {/snippet}
</ExpandContainer>

<style>
    .data {
        display: flex;
        align-items: center;
    }

    .header {
        display: flex;
        align-items: center;
    }
</style>
