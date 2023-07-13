<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import UserInfo from "./UserInfo.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import UserPassword from "./UserPassword.svelte";
    import {slide} from 'svelte/transition';
    import UserDelete from "./UserDelete.svelte";
    import UserForceLogout from "./UserForceLogout.svelte";
    import UserAttr from "./UserAttr.svelte";

    export let user = {};
    export let onSave;

    let isLoading = false;
    let expandContainer;

    const tabBarItems = [
        'INFO',
        'ATTRIBUTES',
        'PASSWORD',
        'LOGOUT',
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
        <Tooltip text="User ID">
            <div class="data font-mono">
                {user.id}
            </div>
        </Tooltip>

        <Tooltip text="E-Mail">
            <div class="data">
                {user.email}
            </div>
        </Tooltip>
    </div>

    <div slot="body">
        <TabBar labels={tabBarItems} bind:selected/>

        {#if selected === 'INFO'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <UserInfo bind:user bind:onSave/>
            </div>
        {:else if selected === 'ATTRIBUTES'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <UserAttr bind:user bind:onSave/>
            </div>
        {:else if selected === 'PASSWORD'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <UserPassword bind:user bind:onSave/>
            </div>
        {:else if selected === 'LOGOUT'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <UserForceLogout bind:user/>
            </div>
        {:else if selected === 'DELETE'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <UserDelete bind:user onSave={onDelete}/>
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
