<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import RoleConfig from "./RoleConfig.svelte";
    import RoleDelete from "./RoleDelete.svelte";

    export let idx = 0;
    export let role = {};
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

<ExpandContainer bind:idx bind:show={expandContainer}>
    <div class="header" slot="header">
        <Tooltip text="Role ID">
            <div class="data font-mono">
                {role.id}
            </div>
        </Tooltip>

        <Tooltip text="Role Name">
            <div class="data">
                {role.name}
            </div>
        </Tooltip>
    </div>

    <div slot="body">
        <TabBar labels={tabBarItems} bind:selected/>

        {#if selected === 'CONFIG'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <RoleConfig bind:role bind:onSave/>
            </div>

        {:else if selected === 'DELETE'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <RoleDelete bind:role onSave={onDelete}/>
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
