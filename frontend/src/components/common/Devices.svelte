<script>
    import {getUserDevices} from "../../utils/dataFetching.js";
    import {onMount} from "svelte";
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import IconCheck from "$lib/icons/IconCheck.svelte";
    import {formatDateFromTs} from "../../utils/helpers.js";
    import Tooltip from "$lib/Tooltip.svelte";
    import IconStop from "$lib/icons/IconStop.svelte";

    export let t;
    export let userId = '';

    let devices = [];

    let formErrors = {};
    let formValues = {};

    onMount(() => {
        fetchDevices();
    })

    async function fetchDevices() {
        let res = await getUserDevices(userId);
        let body = await res.json();
        if (res.ok) {
            for (let device of body) {
                formValues[device.id] = device.name;
            }
            devices = body;
        } else {
            console.error('error fetching devices: ' + body.message);
        }
    }

    async function onSaveName(id) {
        let newName = formValues[id];

        // TODO manual regexp validation for the name

        // TODO only on success
        // update the name in memory to blend out the save button
        formErrors[id] = '';
        for (let device of devices) {
            if (device.id === id) {
                device.name = newName;
                break;
            }
        }
        formValues[id] = newName;
    }

    async function onRevokeRefresh(id) {
        console.error('TODO implemente refresh revoke');

        // TODO on success
        for (let device of devices) {
            if (device.id === id) {
                device.refresh_exp = undefined;
                break;
            }
        }
        devices = [...devices];
    }
</script>

<div class="head">
    {t?.devicesDesc || 'Devices linked to this account'}
</div>

<div class="devices">
    {#each devices as device (device.id)}
        <ExpandContainer>
            <div class="device-header" slot="header">
                <div class="device-head font-mono">
                    {device.name}
                </div>
            </div>

            <div class="device" slot="body">
                <div class="unit">
                    <div class="label font-label">
                        {t?.deviceId.toUpperCase() || 'ID'}
                    </div>
                    <div class="value font-mono">
                        {device.id}
                    </div>
                </div>

                <div class="row">
                    <Input
                            bind:value={formValues[device.id]}
                            bind:error={formErrors[device.id]}
                            autocomplete="off"
                            placeholder={t?.deviceName.toUpperCase() || 'Name'}
                            on:enter={() => onSaveName(device.id)}
                    >
                        {t?.deviceName.toUpperCase() || 'NAME'}
                    </Input>
                    {#if formValues[device.id] != device.name}
                        <Tooltip text={t?.save || 'Save'}>
                            <div
                                    role="button"
                                    tabindex="0"
                                    class="icon-btn-input"
                                    on:click={() => onSaveName(device.id)}
                                    on:keypress={() => onSaveName(device.id)}
                            >
                                <IconCheck color="var(--col-ok)" width={24}/>
                            </div>
                        </Tooltip>
                    {/if}
                </div>

                <div class="unit">
                    <div class="label font-label">
                        {t?.regDate.toUpperCase() || 'REGISTRATION DATE'}
                    </div>
                    <div class="value">
                        {device.created}
                    </div>
                </div>

                <div class="unit">
                    <div class="label font-label">
                        {t?.accessExp.toUpperCase() || 'ACCESS EXPIRES'}
                    </div>
                    <div class="value">
                        {formatDateFromTs(device.access_exp)}
                    </div>
                </div>

                {#if device.refresh_exp}
                    <div class="unit">
                        <div class="label font-label">
                            {t?.accessRenew.toUpperCase() || 'ACCESS RENEW UNTIL'}
                        </div>
                        <div class="row">
                            <div class="value">
                                {formatDateFromTs(device.refresh_exp)}
                            </div>
                            <Tooltip text={t?.accessRenewDelete || 'Delete the possibility to renew'}>
                                <div
                                        role="button"
                                        tabindex="0"
                                        class="icon-btn-value"
                                        on:click={() => onRevokeRefresh(device.id)}
                                        on:keypress={() => onRevokeRefresh(device.id)}
                                >
                                    <IconStop color='var(--col-err)' width={24}/>
                                </div>
                            </Tooltip>
                        </div>
                    </div>
                {/if}

                <div class="unit">
                    <div class="label font-label">
                        {t?.regIp.toUpperCase() || 'REGISTRATION FROM IP'}
                    </div>
                    <div class="value">
                        {device.peer_ip}
                    </div>
                </div>
            </div>
        </ExpandContainer>
    {/each}
</div>

<style>
    .head {
        margin: .5rem 0;
    }

    .device {
        margin: 0 .5rem;
    }

    .device-header {
        display: flex;
        align-items: center;
    }

    .device-head {
        display: flex;
        align-items: center;
        margin: 3px 10px;
    }

    .devices {
        width: 100%;
        /*border: 1px solid red;*/
    }

    .label {
        margin-top: 5px;
        font-size: .9rem;
    }

    .icon-btn-input {
        margin-top: 8px;
    }

    .icon-btn-value {
        margin-left: 3px;
    }

    .icon-btn-input, .icon-btn-value {
        cursor: pointer;
    }

    .row {
        display: flex;
        align-items: center;
    }

    .unit {
        margin: 7px 5px;
    }

    .value {
        display: flex;
        align-items: center;
    }
</style>
