<script>
    import {run} from 'svelte/legacy';

    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import UserInfo from "$lib5/admin/users/UserInfo.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import UserPassword from "$lib5/admin/users/UserPassword.svelte";
    import {slide} from 'svelte/transition';
    import UserDelete from "$lib5/admin/users/UserDelete.svelte";
    import UserForceLogout from "$lib5/admin/users/UserForceLogout.svelte";
    import UserAttr from "./UserAttr.svelte";
    import UserMfa from "./UserMfa.svelte";
    import {onMount} from "svelte";
    import {getUser} from "../../../utils/dataFetching.js";
    import Loading from "$lib/Loading.svelte";
    import UserDevices from "./UserDevices.svelte";

    /**
     * @typedef {Object} Props
     * @property {string} [userEmail]
     * @property {string} [userId]
     * @property {any} onSave
     */

    /** @type {Props} */
    let {userEmail = '', userId = '', onSave = $bindable()} = $props();

    let user = $state();
    let isLoading = $state(true);
    let expandContainer = $state();

    const tabBarItems = [
        'Info',
        'Attributes',
        'Password',
        'MFA',
        'Devices',
        'Logout',
        'Delete',
    ];
    let selected = $state(tabBarItems[0]);
    const tabBarDur = 200;
    const tabBarDly = tabBarDur / 2;


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

    // only fetch user details when the container is expanded
    run(() => {
        if (expandContainer) {
            fetchUser();
        }
    });
</script>

<ExpandContainer bind:show={expandContainer}>
    {#snippet header()}
        <div class="header">
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
    {/snippet}

    {#snippet body()}
        <div>
            {#if isLoading}
                <Loading/>
            {:else}
                <TabBar labels={tabBarItems} bind:selected/>

                {#if selected === 'Info'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserInfo bind:user bind:onSave/>
                    </div>
                {:else if selected === 'Attributes'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserAttr bind:user bind:onSave/>
                    </div>
                {:else if selected === 'Password'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserPassword bind:user bind:onSave/>
                    </div>
                {:else if selected === 'MFA'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserMfa bind:user bind:onSave/>
                    </div>
                {:else if selected === 'Devices'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserDevices bind:user/>
                    </div>
                {:else if selected === 'Logout'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserForceLogout bind:user/>
                    </div>
                {:else if selected === 'Delete'}
                    <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }}
                         out:slide|global={{ duration: tabBarDur }}>
                        <UserDelete bind:user onSave={onDelete}/>
                    </div>
                {/if}
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
