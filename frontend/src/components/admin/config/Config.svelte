<script>
    import Argon2Params from "./argon2/Argon2Params.svelte";
    import TabBar from "$lib/TabBar.svelte";
    import EncKeys from "./encKeys/EncKeys.svelte";
    import PasswordPolicy from "./password_policy/PasswordPolicy.svelte";
    import JWKs from "./jwks/JWKs.svelte";

    const tabBarItems = [
        'Password Policy',
        'JWKS',
        'Argon2 Parameter',
        'Encryption Keys',
    ];
    let selected = tabBarItems[0];
</script>

<div class="content">
    <div class="container">
        <TabBar labels={tabBarItems} bind:selected/>

        {#if selected === 'Password Policy'}
            <PasswordPolicy/>
        {:else if selected === 'JWKS'}
            <JWKs/>
        {:else if selected === 'Argon2 Parameter'}
            <Argon2Params/>
        {:else if selected === 'Encryption Keys'}
            <EncKeys/>
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
