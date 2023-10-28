<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import ApiKeyConfig from "./ApiKeyConfig.svelte";
    import ApiKeyDelete from "./ApiKeyDelete.svelte";
    import {formatDateFromTs} from "../../../utils/helpers.js";
    import ApiKeySecret from "./ApiKeySecret.svelte";

    export let apiKey = {};
    export let onSave = () => {};

    let expandContainer;

    const tabBarItems = [
        'CONFIG',
        'SECRET',
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

    <div slot="body">
        <TabBar labels={tabBarItems} bind:selected/>

        {#if selected === 'CONFIG'}
            <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                <ApiKeyConfig bind:apiKey bind:onSave/>
            </div>

        {:else if selected === 'SECRET'}
            <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                <ApiKeySecret bind:apiKey />
            </div>

        {:else if selected === 'DELETE'}
            <div in:slide={{ delay: tabBarDly, duration: tabBarDur }} out:slide={{ duration: tabBarDur }}>
                <ApiKeyDelete bind:apiKey onSave={onDelete}/>
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
