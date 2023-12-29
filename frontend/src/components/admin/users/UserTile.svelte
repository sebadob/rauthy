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
    import UserMfa from "./UserMfa.svelte";
    import {onMount} from "svelte";
    import {getUser} from "../../../utils/dataFetching.js";
    import Loading from "$lib/Loading.svelte";

    export let userEmail = '';
    export let userId = '';
    export let onSave;

    let user;
    let isLoading = true;
    let expandContainer;

    const tabBarItems = [
        'INFO',
        'ATTRIBUTES',
        'PASSWORD',
        'MFA',
        'LOGOUT',
        'DELETE',
    ];
    let selected = tabBarItems[0];
    const tabBarDur = 200;
    const tabBarDly = tabBarDur / 2;

    // only fetch user details when the container is expanded
    $: if (expandContainer) {
        fetchUser();
    }

    async function fetchUser() {
        const res = await getUser(userId);
        if (res.ok) {
            user = await res.json();
        } else {
            let err = await res.json();
            console.error(err);
        }
        isLoading = false;
    }

    function onDelete() {
        expandContainer = false;
        onSave();
    }

</script>

<ExpandContainer bind:show={expandContainer}>
    <div class="header" slot="header">
        <Tooltip text="User ID">
            <div class="data font-mono">
                {userId}
            </div>
        </Tooltip>

        <Tooltip text="E-Mail">
            <div class="data">
                {userEmail}
            </div>
        </Tooltip>
    </div>

    <div slot="body">
        {#if isLoading}
            <Loading />
        {:else}
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
            {:else if selected === 'MFA'}
                <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                    <UserMfa bind:user bind:onSave/>
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
