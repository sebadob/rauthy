<script lang="ts">
    import type { IParam } from '$state/param.svelte';
    import { fetchGet, fetchPost } from '$api/fetch';
    import type { KVAccessRequest, KVAccessResponse } from '$api/types/kv';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Button from '$lib/button/Button.svelte';
    import Modal from '$lib/Modal.svelte';
    import Form from '$lib/form/Form.svelte';
    import Expandable from '$lib/Expandable.svelte';
    import KVAccessDetails from '$lib/admin/kv/KVAccessDetails.svelte';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import Input from '$lib/form/Input.svelte';
    import { PATTERN_GROUP } from '$utils/patterns';

    let {
        ns,
    }: {
        ns: IParam;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let keys: KVAccessResponse[] = $state([]);

    let urlAccess = $derived(`/auth/v1/kv/ns/${ns.get()}/access`);

    $effect(() => {
        fetchData();
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        let payload: KVAccessRequest = {
            enabled: params.get('enabled') === 'on',
            name: params.get('name') || undefined,
        };
        let res = await fetchPost<KVAccessResponse>(form.action, payload);
        if (res.body) {
            keys.push(res.body);
            closeModal?.();
        } else {
            err = res.error?.message || 'error adding key';
        }
    }

    async function fetchData() {
        let res = await fetchGet<KVAccessResponse[]>(urlAccess);
        if (res.body) {
            keys = res.body;
        } else {
            err = res.error?.message || 'Fetch error';
        }
    }

    function onDelete(id: string) {
        keys = keys.filter(k => k.id !== id);
    }
</script>

<div class="container">
    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}

    <Button onclick={() => (showModal = true)}>{ta.kv.addNewKey}</Button>
    <Modal bind:showModal bind:closeModal>
        <Form action={urlAccess} {onSubmit}>
            <h3>{ta.kv.addNewKey}</h3>

            <InputCheckbox ariaLabel={ta.common.enabled} name="enabled" checked>
                {ta.common.enabled}
            </InputCheckbox>

            <Input
                name="name"
                label={ta.common.name}
                placeholder={ta.common.name}
                value=""
                pattern={PATTERN_GROUP}
            />

            <div class="btns">
                <Button type="submit">
                    {t.common.save}
                </Button>
                <Button level={3} onclick={() => closeModal?.()}>
                    {t.common.cancel}
                </Button>
            </div>
        </Form>
    </Modal>

    <div class="keys">
        {#if keys.length === 0}
            {ta.common.noEntries}
        {:else}
            {#each keys as key, i (key.id)}
                <Expandable>
                    {#snippet summary()}
                        <div class="summary">
                            {#if key.name}
                                {key.name}
                            {:else}
                                <span class="font-mono">
                                    {key.id}
                                </span>
                            {/if}
                        </div>
                    {/snippet}
                    {#snippet details()}
                        <KVAccessDetails bind:key={keys[i]} {urlAccess} {onDelete} />
                    {/snippet}
                </Expandable>
            {/each}
        {/if}
    </div>
</div>

<style>
    .btns {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .container {
        margin: 1.25rem 0;
    }

    .keys {
        margin: 1rem 0;
        max-width: 50rem;
    }

    .summary {
        height: 2rem;
        display: flex;
        align-items: center;
    }
</style>
