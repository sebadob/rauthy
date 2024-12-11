<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import ApiKeyConfig from "./ApiKeyConfig.svelte";
    import ApiKeyDelete from "./ApiKeyDelete.svelte";
    import {formatDateFromTs} from "../../../utils/helpers.js";
    import ApiKeySecret from "./ApiKeySecret.svelte";

    let { apiKey = $bindable({}), onSave = $bindable(() => {
    }) } = $props();

    let expandContainer = $state();

    const tabBarItems = [
        'Config',
        'Secret',
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
        <div class="header" >
            <div class="data">
                {apiKey.name}
            </div>

            {#if apiKey.expires}
                <Tooltip text="Expiry">
                    <div class="data">
                        {formatDateFromTs(apiKey.expires)}
                    </div>
                </Tooltip>
            {/if}
        </div>
    {/snippet}

    {#snippet body()}
        <div >
            <TabBar labels={tabBarItems} bind:selected/>

            {#if selected === 'Config'}
                <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                    <ApiKeyConfig bind:apiKey bind:onSave/>
                </div>

            {:else if selected === 'Secret'}
                <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                    <ApiKeySecret bind:apiKey/>
                </div>

            {:else if selected === 'Delete'}
                <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                    <ApiKeyDelete bind:apiKey onSave={onDelete}/>
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
