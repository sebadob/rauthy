<script lang="ts">
    import type { KVAccessRequest, KVAccessResponse } from '$api/types/kv';
    import Form from '$lib/form/Form.svelte';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import InputPassword from '$lib/form/InputPassword.svelte';
    import Button from '$lib/button/Button.svelte';
    import Input from '$lib/form/Input.svelte';
    import { PATTERN_GROUP } from '$utils/patterns';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { slide } from 'svelte/transition';
    import { untrack } from 'svelte';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { fetchDelete, fetchPost, fetchPut } from '$api/fetch';
    import IconCheck from '$icons/IconCheck.svelte';
    import HiddenValueArea from '$lib/HiddenValueArea.svelte';

    let {
        key = $bindable(),
        urlAccess,
        onDelete,
    }: {
        key: KVAccessResponse;
        urlAccess: string;
        onDelete: (id: string) => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let name = $state(untrack(() => key.name));
    let success = $state(false);
    let successSecret = $state(false);
    let delConfirm = $state(false);

    let bearer = $derived(`${key.id}$${key.secret}`);
    let curl = $derived(
        `curl -s -H 'Authorization: Bearer ${bearer}' ${window.location.origin}/auth/v1/kv/test | jq`,
    );

    async function deleteKey() {
        let res = await fetchDelete(`${urlAccess}/${key.id}`);
        if (res.error) {
            console.error(res.error);
        } else {
            onDelete(key.id);
        }
    }

    async function generateSecret() {
        let res = await fetchPost<KVAccessResponse>(`${urlAccess}/${key.id}/secret`);
        if (res.body) {
            key.secret = res.body.secret;
            successSecret = true;
            setTimeout(() => {
                successSecret = false;
            }, 2000);
        } else if (res.error) {
            console.error(res.error);
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        let payload: KVAccessRequest = {
            name,
            enabled: key.enabled,
        };
        let res = await fetchPut(form.action, payload);
        if (res.error) {
            console.error(res.error);
        } else {
            key.name = name;
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
        }
    }
</script>

<LabeledValue label="ID">
    {key.id}
</LabeledValue>

<Form action={`${urlAccess}/${key.id}`} {onSubmit}>
    <InputCheckbox ariaLabel={ta.common.enabled} bind:checked={key.enabled}>
        {ta.common.enabled}
    </InputCheckbox>

    <Input
        label={ta.common.name}
        placeholder={ta.common.name}
        bind:value={name}
        pattern={PATTERN_GROUP}
    />
    <div class="save" transition:slide={{ duration: 150 }}>
        <Button type="submit">{t.common.save}</Button>
        {#if success}
            <IconCheck />
        {/if}
    </div>
</Form>

<InputPassword ariaLabel="Secret" label="Secret" value={key.secret} disabled />
<div class="flex gap-05">
    <Button level={2} onclick={generateSecret}>Generate New</Button>
    {#if successSecret}
        <IconCheck />
    {/if}
</div>

{#if key.enabled}
    <div class="test" transition:slide={{ duration: 150 }}>
        <p>{@html ta.kv.accessTestDesc}</p>

        <InputPassword
            ariaLabel="Authorization Bearer"
            label="Authorization Bearer"
            value={bearer}
            disabled
            showCopy
        />
        <HiddenValueArea ariaLabel="curl test-command" rows={5} value={curl} width="100%" />
    </div>
{/if}

<div class="delBtn">
    <div class="alignRight">
        {#if delConfirm}
            <div transition:slide={{ duration: 150 }}>
                <p>{ta.kv.delConfirm}</p>

                <Button level={-1} onclick={deleteKey}>
                    {t.common.delete}
                </Button>
                <Button level={3} onclick={() => (delConfirm = false)}>
                    {t.common.cancel}
                </Button>
            </div>
        {:else}
            <Button level={-1} onclick={() => (delConfirm = true)}>
                {t.common.delete}
            </Button>
        {/if}
    </div>
</div>

<style>
    .alignRight {
        text-align: right;
    }

    .delBtn {
        margin-top: 1rem;
        display: flex;
        justify-content: flex-end;
    }

    .save {
        margin: 0.5rem 0 1rem 0.25rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .test {
        margin-top: 1rem;
    }
</style>
