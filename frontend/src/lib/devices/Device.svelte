<script lang="ts">
    import type { DeviceRequest, DeviceResponse } from '$api/types/device.ts';
    import { formatDateFromTs } from '$utils/helpers';
    import IconStop from '$icons/IconStop.svelte';
    import Input from '$lib5/form/Input.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { untrack } from 'svelte';
    import Expandable from '$lib5/Expandable.svelte';
    import { PATTERN_CLIENT_NAME } from '$utils/patterns';
    import LabeledValue from '$lib5/LabeledValue.svelte';
    import Button from '$lib5/button/Button.svelte';
    import { fetchDelete, fetchPut } from '$api/fetch';

    let {
        device = $bindable(),
        userId,
    }: {
        device: DeviceResponse;
        userId: string;
    } = $props();

    let t = useI18n();

    let name = $state(untrack(() => device.name));

    async function onSaveName() {
        let payload: DeviceRequest = {
            device_id: device.id,
            name,
        };
        let res = await fetchPut(`/auth/v1/users/${userId}/devices`, payload);
        if (!res.error) {
            // update the name in memory to blend out the save button
            device.name = name;
        } else {
            console.error(res.error);
        }
    }

    async function onRevokeRefresh() {
        let payload: DeviceRequest = {
            device_id: device.id,
        };
        let res = await fetchDelete(`/auth/v1/users/${userId}/devices`, payload);
        if (!res.error) {
            device.refresh_exp = undefined;
        } else {
            console.error(res.error);
        }
    }
</script>

<Expandable>
    {#snippet summary()}
        <div class="device-header">
            <div class="device-head font-mono">
                {device.name}
            </div>
        </div>
    {/snippet}

    {#snippet details()}
        <div class="device">
            <LabeledValue
                label={t.account.deviceId}
                mono
            >
                {device.id}
            </LabeledValue>

            <div class="row">
                <Input
                    bind:value={name}
                    autocomplete="off"
                    width="17rem"
                    label={t.account.deviceName}
                    placeholder={t.account.deviceName}
                    onEnter={onSaveName}
                    pattern={PATTERN_CLIENT_NAME}
                    required
                >
                    {t.account.deviceName}
                </Input>
                {#if name !== device.name}
                    <div class="saveButton">
                        <Button onclick={onSaveName}>
                            {t.common.save}
                        </Button>
                    </div>
                {/if}
            </div>

            <LabeledValue label={t.account.regDate}>
                {formatDateFromTs(device.created)}
            </LabeledValue>

            <LabeledValue label={t.account.accessExp}>
                {formatDateFromTs(device.access_exp)}
            </LabeledValue>

            {#if device.refresh_exp}
                <LabeledValue label={t.account.accessRenew}>
                    <div>
                        {formatDateFromTs(device.refresh_exp)}
                    </div>
                    {#snippet button()}
                        <Button
                            ariaLabel={t.common.delete}
                            invisible
                            onclick={onRevokeRefresh}
                        >
                            <div title={t.common.delete}>
                                <IconStop />
                            </div>
                        </Button>
                    {/snippet}
                </LabeledValue>
            {/if}

            <LabeledValue label={t.account.regIp}>
                {device.peer_ip}
            </LabeledValue>
        </div>
    {/snippet}
</Expandable>

<style>
    .device {
        margin: 0 0.5rem;
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

    .row {
        display: flex;
        gap: 0.66rem;
        flex-wrap: wrap;
    }

    .saveButton {
        margin-top: 0.5rem;
    }
</style>
