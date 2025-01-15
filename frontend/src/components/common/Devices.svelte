<script>
    import {getUserDevices, putUserDeviceName, deleteUserDeviceRefresh} from "../../utils/dataFetching.js";
    import {onMount} from "svelte";
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import IconCheck from "$lib/icons/IconCheck.svelte";
    import {formatDateFromTs} from "../../utils/helpers";
    import Tooltip from "$lib/Tooltip.svelte";
    import IconStop from "$lib/icons/IconStop.svelte";
    import {REGEX_CLIENT_NAME} from "../../utils/constants.js";

    /**
     * @typedef {Object} Props
     * @property {any} t
     * @property {string} [userId]
     */

    /** @type {Props} */
    let {t, userId = ''} = $props();

    let devices = $state([]);

    let formErrors = $state({});
    let formValues = $state({});

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

        let isValid = REGEX_CLIENT_NAME.test(newName);
        if (isValid) {
            formErrors[id] = '';
        } else {
            formErrors[id] = t?.invalidInput || 'Invalid Input';
            return;
        }

        let data = {
            device_id: id,
            name: newName,
        };
        let res = await putUserDeviceName(userId, data);
        if (res.ok) {
            // update the name in memory to blend out the save button
            formErrors[id] = '';
            for (let device of devices) {
                if (device.id === id) {
                    device.name = newName;
                    break;
                }
            }
            devices = [...devices];
            formValues[id] = newName;
        } else {
            let body = await res.json();
            console.error(body);
        }

    }

    async function onRevokeRefresh(id) {
        let data = {
            device_id: id,
        };
        let res = await deleteUserDeviceRefresh(userId, data);
        if (res.ok) {
            for (let device of devices) {
                if (device.id === id) {
                    device.refresh_exp = undefined;
                    break;
                }
            }
            devices = [...devices];
        } else {
            let body = await res.json();
            console.error(body);
        }

    }
</script>

<div class="head">
    {t?.devicesDesc || 'Devices linked to this account'}
</div>

<div class="devices">
    {#each devices as device (device.id)}
        <ExpandContainer>
            {#snippet header()}
                <div class="device-header">
                    <div class="device-head font-mono">
                        {device.name}
                    </div>
                </div>
            {/snippet}

            {#snippet body()}
                <div class="device">
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
                                        onclick={() => onSaveName(device.id)}
                                        onkeypress={() => onSaveName(device.id)}
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
                                            onclick={() => onRevokeRefresh(device.id)}
                                            onkeypress={() => onRevokeRefresh(device.id)}
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
            {/snippet}
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
