<script lang="ts">
    import {onMount} from "svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useSession} from "$state/session.svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {DeviceResponse} from "$api/types/device.ts";
    import Device from "./Device.svelte";

    let {
        viewMode,
    }: {
        viewMode: 'admin' | 'account',
    } = $props();

    let t = useI18n();
    let session = useSession(viewMode);
    let userId = $derived(session.get()?.user_id);

    let devices: DeviceResponse[] = $state([]);

    onMount(() => {
        fetchDevices();
    })

    async function fetchDevices() {
        let res = await fetchGet<DeviceResponse[]>(`/auth/v1/users/${userId}/devices`);
        if (res.body) {
            devices = res.body;
        } else {
            console.error('error fetching devices: ' + res.error);
        }
    }
</script>

<div class="head">
    {t?.account.devicesDesc || 'Devices linked to this account'}
</div>

<div class="devices">
    {#if userId}
        {#each devices as device, i (device.id)}
            <Device bind:device={devices[i]} {userId}/>
        {/each}
    {/if}
</div>

<style>
    .head {
        margin: .5rem 0;
    }

    .devices {
        width: 100%;
    }
</style>
