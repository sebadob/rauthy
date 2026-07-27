<script lang="ts">
    import { onMount } from 'svelte';
    import { fetchGet, fetchPost } from '$api/fetch';
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import NavSub from '$lib5/nav/NavSub.svelte';
    import { useParam } from '$state/param.svelte';
    import NavButtonTile from '$lib5/nav/NavButtonTile.svelte';
    import ButtonAddModal from '$lib5/button/ButtonAddModal.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useTrigger } from '$state/callback.svelte';
    import type { KVNamespaceRequest, KVNamespaceResponse } from '$api/types/kv';
    import KVDetails from '$lib/admin/kv/KVDetails.svelte';
    import Tabs from '$lib/tabs/Tabs.svelte';
    import KVAccess from '$lib/admin/kv/KVAccess.svelte';
    import KVDelete from '$lib/admin/kv/KVDelete.svelte';
    import Form from '$lib/form/Form.svelte';
    import Input from '$lib/form/Input.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { PATTERN_GROUP } from '$utils/patterns';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import KVNamespaceEdit from '$lib/admin/kv/KVNamespaceEdit.svelte';

    let t = useI18n();
    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let refInput: undefined | HTMLInputElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();

    let ns = useParam('ns');

    let err = $state('');

    // order: data, access, delete
    let tab = $state(ta.kv.tabs[0]);
    let namespaces: KVNamespaceResponse[] = $state([]);
    let selected: undefined | KVNamespaceResponse = $derived(
        namespaces.find(namespace => namespace.name === ns.get()),
    );

    onMount(() => {
        fetchData();
    });

    $effect(() => {
        if (refInput) {
            requestAnimationFrame(() => {
                refInput?.focus();
            });
        }
    });

    async function fetchData() {
        let res = await fetchGet<KVNamespaceResponse[]>('/auth/v1/kv/ns');
        if (res.body) {
            namespaces = res.body;
            if (!ns.get() && res.body.length > 0) {
                ns.set(res.body[0].name);
            }
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onDelete() {
        ns.set(undefined);
        closeModal?.();
        fetchData();
        tab = ta.kv.tabs[0];
    }

    async function onSave(nameNew: string) {
        await fetchData();
        ns.set(nameNew);
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        let payload: KVNamespaceRequest = {
            name: params.get('name') || '',
            public: params.get('public') === 'on',
        };
        let res = await fetchPost(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            await fetchData();
        }
        closeModal?.();
    }
</script>

<NavSub width="min(16rem, 100dvw - .5rem)" buttonTilesAriaControls="kv" paddingTop="2.35rem">
    <ButtonAddModal
        bind:ref={refAddNew}
        level={namespaces.length === 0 ? 1 : 2}
        bind:closeModal
        alignRight
    >
        <div class="alignLeft">
            <Form action="/auth/v1/kv/ns" {onSubmit}>
                <h3>{ta.kv.addNewNs}</h3>
                <Input
                    bind:ref={refInput}
                    name="name"
                    label={ta.common.name}
                    placeholder={ta.common.name}
                    pattern={PATTERN_GROUP}
                />
                <InputCheckbox ariaLabel="Public Access" name="public" checked={false}>
                    Public Access
                </InputCheckbox>

                <div class="btns">
                    <Button type="submit">
                        {t.common.save}
                    </Button>
                    <Button level={3} onclick={() => closeModal?.()}>
                        {t.common.cancel}
                    </Button>
                </div>
            </Form>
        </div>
    </ButtonAddModal>

    {#snippet buttonTiles()}
        <div class="nsList">
            {#each namespaces as namespace (namespace.name)}
                <NavButtonTile
                    onclick={() => ns.set(namespace.name)}
                    selected={ns.get() === namespace.name}
                >
                    <div style:margin-top=".25rem">
                        {namespace.name}
                        {#if namespace.public}
                            <i>(public)</i>
                        {/if}
                    </div>
                </NavButtonTile>
            {/each}
        </div>
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="kv" aria-label={ta.common.details}>
        {#if selected}
            <div class="tabs">
                <Tabs tabs={ta.kv.tabs} bind:selected={tab} />
            </div>

            {#if tab === ta.kv.tabs[0]}
                <KVDetails ns={selected} />
            {:else if tab === ta.kv.tabs[1]}
                <KVAccess {ns} />
            {:else if tab === ta.kv.tabs[2]}
                <KVNamespaceEdit ns={selected} {onSave} />
            {:else if tab === ta.kv.tabs[3]}
                <KVDelete {ns} {onDelete} />
            {/if}
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</ContentAdmin>

<style>
    #kv {
        max-width: 60rem;
    }

    .alignLeft {
        text-align: left;
    }

    .btns {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .nsList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }

    .tabs {
        display: flex;
        margin-bottom: 0.5rem;
    }
</style>
