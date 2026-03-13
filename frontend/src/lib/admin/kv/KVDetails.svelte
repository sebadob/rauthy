<script lang="ts">
    import type { IParam } from '$state/param.svelte';
    import type { KVValueRequest, KVValueResponse } from '$api/types/kv';
    import Button from '$lib/button/Button.svelte';
    import Input from '$lib/form/Input.svelte';
    import SearchBar from '$lib/search_bar/SearchBar.svelte';
    import { fetchDelete, fetchGet, fetchPost, fetchPut } from '$api/fetch';
    import Modal from '$lib/Modal.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Form from '$lib/form/Form.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import InputArea from '$lib/form/InputArea.svelte';
    import { PATTERN_GROUP } from '$utils/patterns';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { slide } from 'svelte/transition';
    import { parseJsonValue, stringifyJsonValue } from '$utils/jsonValue';
    import IconKey from '$icons/IconKey.svelte';
    import IconEdit from '$icons/IconEdit.svelte';
    import IconTrash from '$icons/IconTrash.svelte';

    let {
        ns,
    }: {
        ns: IParam;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let showModalAdd = $state(false);
    let closeModalAdd: undefined | (() => void) = $state();
    let showModalEdit = $state(false);
    let closeModalEdit: undefined | (() => void) = $state();
    let showModalDelete = $state(false);
    let closeModalDelete: undefined | (() => void) = $state();

    let values: KVValueResponse[] = $state([]);
    let valuesSearch: KVValueResponse[] = $state([]);
    let entrySelected: KVValueResponse | undefined = $state();
    let limit = $state(1000);
    let err = $state('');
    let search = $state('');

    let encryptNew = $state(false);

    $effect(() => {
        // make sure to reset on nav-switch
        if (ns.get()) {
            values = [];
        }
    });

    $effect(() => {
        if (search.length > 2) {
            searchValues();
        } else {
            valuesSearch = [];
        }
    });

    function editEntry(entry: KVValueResponse) {
        entrySelected = entry;
        showModalEdit = true;
    }

    function deleteEntry(entry: KVValueResponse) {
        entrySelected = entry;
        showModalDelete = true;
    }

    async function fetchAll() {
        if (err) {
            err = '';
        }
        if (search) {
            search = '';
        }

        let res = await fetchGet<KVValueResponse[]>(
            `/auth/v1/kv/ns/${ns.get()}/values?limit=${limit}`,
        );
        if (res.body) {
            res.body.sort((a, b) => a.key.localeCompare(b.key));
            values = res.body;
        } else {
            err = res.error?.message || 'Failed to fetch values';
        }
    }

    async function searchValues() {
        if (err) {
            err = '';
        }

        let res = await fetchGet<KVValueResponse[]>(
            `/auth/v1/kv/ns/${ns.get()}/values?limit=${limit}&search=${search}`,
        );
        if (res.body) {
            res.body.sort((a, b) => a.key.localeCompare(b.key));
            valuesSearch = res.body;
        } else {
            err = res.error?.message || 'Failed to fetch values';
        }
    }

    async function onSubmitAdd(form: HTMLFormElement, params: URLSearchParams) {
        let value = params.get('value')?.trim();
        if (!value) {
            return;
        }

        let payload: KVValueRequest = {
            key: params.get('key') || '',
            encrypted: !!params.get('encrypted'),
            value: parseJsonValue(value),
        };

        let res = await fetchPost(form.action, payload);
        if (res.error) {
            err = res.error.message || 'Failed to add key';
        } else {
            closeModalAdd?.();
            if (values.length > 0) {
                await fetchAll();
            } else {
                search = payload.key;
            }
        }
    }

    async function onSubmitEdit(form: HTMLFormElement, params: URLSearchParams) {
        if (!entrySelected) {
            return;
        }
        let value = params.get('value')?.trim();
        if (!value) {
            return;
        }

        let payload: KVValueRequest = {
            key: entrySelected.key,
            encrypted: entrySelected.encrypted,
            value: parseJsonValue(value),
        };

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message || 'Failed to update key';
        } else {
            closeModalEdit?.();

            if (valuesSearch.length > 0) {
                valuesSearch = valuesSearch.map(v => {
                    if (v.key === entrySelected?.key) {
                        v.value = value;
                    }
                    return v;
                });
            }
            if (values.length > 0) {
                values = values.map(v => {
                    if (v.key === entrySelected?.key) {
                        v.value = value;
                    }
                    return v;
                });
            }
        }
    }

    async function onSubmitDelete() {
        if (!entrySelected) {
            return;
        }

        let res = await fetchDelete(`/auth/v1/kv/ns/${ns.get()}/values/${entrySelected?.key}`);
        if (res.error) {
            err = res.error.message;
        } else {
            closeModalDelete?.();

            if (valuesSearch.length > 0) {
                valuesSearch = valuesSearch.filter(v => v.key !== entrySelected?.key);
            }
            if (values.length > 0) {
                values = values.filter(v => v.key !== entrySelected?.key);
            }
        }
    }
</script>

{#snippet error()}
    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
{/snippet}

<div class="btnsLim">
    <Input typ="number" bind:value={limit} label="Limit" min="1" max="4294967295" width="7rem" />
    <div class="btns">
        <Button level={2} onclick={fetchAll}>{ta.kv.loadAllValues}</Button>
        <Button onclick={() => (showModalAdd = true)}>{ta.kv.addNew}</Button>
        <Modal bind:showModal={showModalAdd} bind:closeModal={closeModalAdd}>
            <div class="modal">
                <Form action={`/auth/v1/kv/ns/${ns.get()}/values`} onSubmit={onSubmitAdd}>
                    <Input
                        name="key"
                        label={ta.kv.key}
                        placeholder={ta.kv.key}
                        pattern={PATTERN_GROUP}
                        required
                    />
                    <InputArea
                        name="value"
                        label={ta.kv.value}
                        placeholder={ta.kv.value}
                        rows={15}
                        required
                    />
                    <InputCheckbox
                        ariaLabel={ta.kv.storeEncrypted}
                        name="encrypted"
                        bind:checked={encryptNew}
                    >
                        {ta.kv.storeEncrypted}
                    </InputCheckbox>
                    {#if encryptNew}
                        <div transition:slide={{ duration: 150 }}>
                            <p>{ta.kv.encryptedDesc}</p>
                        </div>
                    {/if}

                    <div class="btnModal">
                        <Button type="submit">{t.common.save}</Button>
                        <Button level={3} onclick={() => closeModalAdd?.()}>
                            {t.common.cancel}
                        </Button>
                    </div>

                    {@render error()}
                </Form>
            </div>
        </Modal>
    </div>
</div>

<SearchBar bind:value={search} />

{@render error()}

{#snippet renderEntry(entry: KVValueResponse)}
    <div class="row">
        <div class="key">
            <div>
                {entry.key}
            </div>
            <div class="flex gap-05">
                {#if entry.encrypted}
                    <div title={ta.kv.storeEncrypted} style:margin-bottom="-.4rem">
                        <IconKey width="1.1rem" />
                    </div>
                {/if}
                <Button ariaLabel={ta.common.edit} invisible onclick={() => editEntry(entry)}>
                    <IconEdit width="1.25rem" />
                </Button>
                <Button ariaLabel={t.common.delete} invisible onclick={() => deleteEntry(entry)}>
                    <IconTrash width="1.25rem" />
                </Button>
            </div>
        </div>
        <div class="value">
            {stringifyJsonValue(entry.value)}
        </div>
    </div>
{/snippet}

<div class="values">
    {#if valuesSearch.length > 0}
        {#each valuesSearch as entry (entry.key)}
            {@render renderEntry(entry)}
        {/each}
    {:else if values.length > 0 && search.length < 3}
        {#each values as entry (entry.key)}
            {@render renderEntry(entry)}
        {/each}
    {:else}
        <div class="m-05">
            {ta.common.noEntries}
        </div>
    {/if}

    <Modal
        bind:showModal={showModalEdit}
        bind:closeModal={closeModalEdit}
        onClose={() => (entrySelected = undefined)}
    >
        <div class="modal">
            {#if entrySelected}
                <Form action={`/auth/v1/kv/ns/${ns.get()}/values`} onSubmit={onSubmitEdit}>
                    <h3>
                        {entrySelected.key}
                    </h3>
                    <InputArea
                        name="value"
                        label={ta.kv.value}
                        placeholder={ta.kv.value}
                        rows={15}
                        value={stringifyJsonValue(entrySelected.value)}
                        required
                    />
                    <InputCheckbox
                        ariaLabel={ta.kv.storeEncrypted}
                        bind:checked={entrySelected.encrypted}
                    >
                        {ta.kv.storeEncrypted}
                    </InputCheckbox>
                    {#if entrySelected.encrypted}
                        <div transition:slide={{ duration: 150 }}>
                            <p>{ta.kv.encryptedDesc}</p>
                        </div>
                    {/if}

                    <div class="btnModal">
                        <Button type="submit">{t.common.save}</Button>
                        <Button level={3} onclick={() => closeModalEdit?.()}>
                            {t.common.cancel}
                        </Button>
                    </div>
                </Form>
            {/if}
        </div>
    </Modal>
    <Modal
        bind:showModal={showModalDelete}
        bind:closeModal={closeModalDelete}
        onClose={() => (entrySelected = undefined)}
    >
        <div class="modal">
            {#if entrySelected}
                <h3>
                    {t.common.delete}
                </h3>
                <p>
                    {ta.kv.deleteConfirmMsg.replace('{{ key }}', entrySelected.key)}
                </p>
                <div class="btnModal">
                    <Button level={-1} onclick={onSubmitDelete}>{t.common.delete}</Button>
                    <Button level={3} onclick={() => closeModalDelete?.()}>
                        {t.common.cancel}
                    </Button>
                </div>
            {/if}
        </div>
    </Modal>
</div>

<style>
    .btnModal {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .btns {
        margin-top: 1.35rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .btnsLim {
        display: flex;
        gap: 0.5rem;
    }

    .modal {
        width: min(90dvw, 40rem);
    }

    .row {
        padding: 0.25rem 0.5rem;
        border: 1px solid transparent;
        border-radius: var(--border-radius);
    }

    .key {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: hsl(var(--text-high));
    }

    .row:nth-child(even) {
        background-color: hsla(var(--bg-high) / 0.25);
    }

    .row:hover {
        border-color: hsl(var(--accent));
    }

    .value {
        white-space: pre-wrap;
    }

    .values {
        max-height: calc(100dvh - 13rem);
        margin: 0.5rem 0;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        overflow: scroll;
    }
</style>
