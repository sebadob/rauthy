<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import GroupConfig from "./GroupConfig.svelte";
    import GroupDelete from "./GroupDelete.svelte";

    let {group = $bindable({}), onSave = $bindable()} = $props();

    let isLoading = false;
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
            <Tooltip text="Group ID">
                <div class="data font-mono">
                    {group.id}
                </div>
            </Tooltip>

            <Tooltip text="Group Name">
                <div class="data">
                    {group.name}
                </div>
            </Tooltip>
        </div>
    {/snippet}

    {#snippet body()}
        <div>
            <TabBar labels={tabBarItems} bind:selected/>

            {#if selected === 'Config'}
                <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                     out:slide|global={{ duration: tabBarDur }}>
                    <GroupConfig bind:group {onSave}/>
                </div>

            {:else if selected === 'Delete'}
                <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                     out:slide|global={{ duration: tabBarDur }}>
                    <GroupDelete {group} onSave={onDelete}/>
                </div>
            {/if}
        </div>
    {/snippet}
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
