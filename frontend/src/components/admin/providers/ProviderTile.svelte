<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import ProviderConfig from "./ProviderConfig.svelte";

    export let provider = {};
    export let onSave;

    let isLoading = false;
    let expandContainer;

    const tabBarItems = [
        'CONFIG',
        'DELETE',
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

        {#if selected === 'CONFIG'}
            <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                <ProviderConfig bind:provider bind:onSave/>
            </div>

        {:else if selected === 'DELETE'}
            <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                <!--                <ScopeDelete bind:scope onSave={onDelete}/>-->
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
