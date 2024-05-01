<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import ProviderConfig from "./ProviderConfig.svelte";
    import ProviderDelete from "./ProviderDelete.svelte";

    export let provider = {};
    export let onSave;

    let expandContainer;

    const tabBarItems = [
        'Config',
        'Delete',
    ];
    let selected = tabBarItems[0];
    const tabBarDur = 200;
    const tabBarDly = tabBarDur / 2;

    function onDelete() {
        expandContainer = false;
        onSave();
    }

</script>

<ExpandContainer bind:show={expandContainer}>
    <div class="header" slot="header">
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

    <div slot="body">
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
</ExpandContainer>

<style>
    .data {
        display: flex;
        align-items: center;
        margin: 3px 10px;
    }

    .header {
        display: flex;
        align-items: center;
    }
</style>
