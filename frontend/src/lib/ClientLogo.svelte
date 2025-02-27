<script lang="ts">
    let {
        clientId,
    }: {
        clientId: string,
    } = $props();

    console.log('rendering clientLogo');
</script>

<div class="logo">
    <img src="{`/auth/v1/clients/${clientId}/logo`}" alt="No Logo Available"/>
</div>

<style>
    .logo {
        margin: 0 .25rem;
        width: 84px;
        height: 84px;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>