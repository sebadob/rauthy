<script lang="ts">
    import Devices from "$lib5/devices/Devices.svelte";
    import {useSession} from "$state/session.svelte.ts";

    let session = useSession('account');
    let userId = $derived(session.get()?.user_id);
</script>

{#if userId}
    <Devices viewMode="account" {userId}/>
{/if}
