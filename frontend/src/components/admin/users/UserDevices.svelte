<script>

    import Devices from "../../common/Devices.svelte";

    export let user;

</script>

<div class="container">
    <Devices userId={user.id}/>
</div>

<style>
    .container {
        margin-bottom: 1rem;
        padding-left: 1rem;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }
</style>
