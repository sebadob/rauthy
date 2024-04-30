<script>
    import Devices from "../common/Devices.svelte";

    export let t;
    export let sessionInfo;
    export let user = {};

</script>

<div class="container">
    <!--
    This is component is only a wrapper because the same Devices
    is reused in the admin ui
    -->
    <Devices bind:t bind:userId={sessionInfo.user_id}/>
</div>

<style>
    .container {
        padding-left: 10px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }
</style>
