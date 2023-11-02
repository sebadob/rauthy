<script>
    import Argon2Params from "./argon2/Argon2Params.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {slide} from 'svelte/transition';
    import EncKeys from "./encKeys/EncKeys.svelte";
    import PasswordPolicy from "./password_policy/PasswordPolicy.svelte";
    import JWKs from "./jwks/JWKs.svelte";

    const tabBarItems = [
        'PASSWORD POLICY',
        'JWKS',
        'ARGON2 PARAMETERS',
        'ENCRYPTION KEYS',
    ];
    let selected = tabBarItems[0];
    const tabBarDur = 200;
    const tabBarDly = tabBarDur / 2;
</script>

<div class="content">
    <div class="container">
        <TabBar labels={tabBarItems} bind:selected />

        {#if selected === 'PASSWORD POLICY'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <PasswordPolicy/>
            </div>

        {:else if selected === 'JWKS'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <JWKs/>
            </div>

        {:else if selected === 'ARGON2 PARAMETERS'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <Argon2Params/>
            </div>

        {:else if selected === 'ENCRYPTION KEYS'}
            <div in:slide|global={{ delay: tabBarDly, duration: tabBarDur }} out:slide|global={{ duration: tabBarDur }}>
                <EncKeys/>
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 1280px;
        max-height: 90vh;
        overflow-y: auto;
    }
</style>
