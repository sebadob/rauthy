<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte.js';
    import { fetchGet, fetchPost } from '$api/fetch.js';
    import { onMount } from 'svelte';
    import Button from '$lib/button/Button.svelte';
    import Modal from '$lib/Modal.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import type { ToSRequest, ToSResponse, ToSUserAcceptResponse } from '$api/types/tos';
    import SearchBar from '$lib/search_bar/SearchBar.svelte';
    import { fetchSearchServer } from '$utils/search';
    import type { UserResponseSimple } from '$api/types/user';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { slide } from 'svelte/transition';
    import InputDateTimeCombo from '$lib/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput } from '$utils/form';
    import { formatDateFromTs, formatUtcTsFromDateInput } from '$utils/helpers';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import Options from '$lib/Options.svelte';
    import EditorInteractive from '$lib/text_edit/EditorInteractive.svelte';

    let t = useI18n();
    let ta = useI18nAdmin();

    let refSearch: undefined | HTMLInputElement = $state();

    let showModalAddNew = $state(false);
    let closeModalAddNew: undefined | (() => void) = $state();
    let showModalStatus = $state(false);
    let closeModalStatus: undefined | (() => void) = $state();
    let isModalOpen = $derived(showModalAddNew || showModalStatus);

    let error = $state('');
    let noneExist = $state(false);
    let tos: ToSResponse[] = $state([]);
    let editorMode: 'HTML' | 'Text' | 'Markdown' = $state('Markdown');
    let newToSContent = $state('');
    let newToSContentRaw = $state('');
    let optUntil = $state(false);
    let optUntilDate = $state(fmtDateInput());
    let optUntilTime = $state(fmtTimeInput());

    let selectOpts: string[] = $state([]);
    let selectedTsLabel = $state('');
    let selectedIdx = $state(-1);

    let searchValue = $state('');
    let searchOptions: UserResponseSimple[] = $state([]);
    let selectedId = $state('');
    let selectedEmail = $state('');
    let userStatus: undefined | ToSUserAcceptResponse[] = $state();

    onMount(() => {
        getTos();
    });

    $effect(() => {
        getUserStatus();
    });

    $effect(() => {
        searchUser();
    });

    $effect(() => {
        if (tos.length > 0) {
            let opts = tos.map(tos => formatDateFromTs(tos.ts));
            opts[0] = `${opts[0]} (Latest)`;

            selectOpts = opts;
            selectedTsLabel = opts[0];
        } else {
            selectOpts = [];
            selectedTsLabel = '';
        }
    });

    $effect(() => {
        if (selectedTsLabel) {
            selectedIdx = selectOpts.findIndex(o => o === selectedTsLabel);
        } else {
            selectedIdx = -1;
        }
    });

    $effect(() => {
        if (refSearch) {
            requestAnimationFrame(() => {
                refSearch?.focus();
            });
        }
    });

    $effect(() => {
        if (searchOptions.length === 1) {
            selectOpt(searchOptions[0]);
        }
    });

    async function searchUser() {
        if (searchValue.length < 3) {
            searchOptions = [];
            return;
        }

        let res = await fetchSearchServer<UserResponseSimple[]>({
            ty: 'user',
            idx: 'email',
            q: searchValue,
        });
        if (res.body) {
            searchOptions = res.body;
        } else {
            console.error(res.error);
        }
    }

    async function getUserStatus() {
        if (!selectedId) {
            userStatus = undefined;
            return;
        }

        let res = await fetchGet<ToSUserAcceptResponse[]>(`/auth/v1/tos/user/${selectedId}`);
        if (res.body) {
            userStatus = res.body;
        } else {
            console.error(res.error);
        }
    }

    async function getTos() {
        let res = await fetchGet<ToSResponse[]>('/auth/v1/tos');
        if (res.body) {
            // Always have the latest ToS at the top. The backend sends them ASC.
            res.body.reverse();
            tos = res.body;
        } else if (res.status === 204) {
            noneExist = true;
            selectedIdx = -1;
        }
    }

    async function saveToS() {
        error = '';

        let content = newToSContent.trim();
        if (!newToSContent || content.length === 0) {
            return;
        }

        let payload: ToSRequest = {
            is_html: editorMode !== 'Text',
            content,
        };
        if (optUntil) {
            payload.opt_until = formatUtcTsFromDateInput(optUntilDate, optUntilTime);
        }

        let res = await fetchPost('/auth/v1/tos', payload);
        if (res.error) {
            error = res.error.message;
        } else {
            closeModalAddNew?.();
            // we need a short timeout for the very first ToS
            setTimeout(() => {
                getTos();
            }, 500);
        }
    }

    function selectOpt(opt: UserResponseSimple) {
        selectedEmail = opt.email;
        selectedId = opt.id;
    }
</script>

<h2>{t.tos.tos}</h2>

<div class="flex gap-05 flex-wrap mh-10">
    <Button
        ariaLabel={ta.tos.addNewToS}
        level={isModalOpen ? 2 : 1}
        onclick={() => (showModalAddNew = true)}
    >
        {ta.tos.addNewToS}
    </Button>
    <Button
        ariaLabel={ta.tos.addNewToSFromCurrent}
        level={isModalOpen ? 2 : 1}
        onclick={() => {
            newToSContentRaw = selectedIdx > -1 && tos.length > 0 ? tos[selectedIdx].content : '';
            showModalAddNew = true;
        }}
    >
        {ta.tos.addNewToSFromCurrent}
    </Button>
    {#if tos.length > 0}
        <Button
            ariaLabel={ta.tos.checkStatus}
            level={isModalOpen ? 3 : 2}
            onclick={() => (showModalStatus = true)}
        >
            {ta.tos.checkStatus}
        </Button>
    {/if}
</div>

<div>
    {#if noneExist}
        <p>{ta.tos.noneExist}</p>
    {:else if tos.length > 0 && selectedIdx > -1}
        {@const active = tos[selectedIdx]}

        <div>
            {#if selectedTsLabel && selectOpts.length > 1}
                <Options ariaLabel="Select ToS" options={selectOpts} bind:value={selectedTsLabel} />
            {/if}

            <LabeledValue label={ta.tos.added} title={ta.tos.added}>
                {formatDateFromTs(active.ts)}
            </LabeledValue>

            {#if active.opt_until}
                <LabeledValue label={ta.tos.optUntil.label} title={ta.tos.optUntil.label}>
                    {formatDateFromTs(active.opt_until)}
                </LabeledValue>
            {/if}
        </div>

        <div class="html-render">
            {#if active.is_html}
                {@html active.content}
            {:else}
                {active.content}
            {/if}
        </div>
    {/if}
</div>

<Modal bind:showModal={showModalAddNew} bind:closeModal={closeModalAddNew}>
    <div class="modal">
        <h3>{ta.tos.addNewToS}</h3>

        <div class="optUntil">
            <InputCheckbox ariaLabel={ta.tos.optUntil.enable} bind:checked={optUntil}>
                {ta.tos.optUntil.enable}
            </InputCheckbox>

            {#if optUntil}
                <div transition:slide={{ duration: 150 }}>
                    <InputDateTimeCombo
                        min={fmtDateInput()}
                        timeMin={fmtTimeInput()}
                        withTime
                        bind:value={optUntilDate}
                        bind:timeValue={optUntilTime}
                    />

                    <p>
                        {ta.tos.optUntil.desc}
                    </p>
                </div>
            {/if}
        </div>

        <div class="editor">
            <EditorInteractive
                bind:mode={editorMode}
                bind:contentRaw={newToSContentRaw}
                bind:sanitizedValue={newToSContent}
                height="max(calc(100dvh - 20rem), 30rem)"
            />
        </div>

        <div class="action" style:margin-bottom="1rem">
            <p><b>{ta.tos.immutable}</b></p>
        </div>

        <Button
            ariaLabel={t.common.save}
            onclick={saveToS}
            isDisabled={!newToSContent || newToSContent.trim().length === 0}
        >
            {t.common.save}
        </Button>
        <Button level={-3} ariaLabel={t.common.cancel} onclick={() => closeModalAddNew?.()}>
            {t.common.cancel}
        </Button>

        {#if error}
            <div class="err">
                {error}
            </div>
        {/if}
    </div>
</Modal>

{#if tos.length > 0}
    <Modal bind:showModal={showModalStatus} bind:closeModal={closeModalStatus}>
        <h3>{ta.tos.checkStatus}</h3>

        <div class="userSearch">
            <SearchBar bind:ref={refSearch} placeholder="E-Mail" bind:value={searchValue} />
            <div class="searchOpts">
                {#each searchOptions as opt}
                    <Button level={3} onclick={() => selectOpt(opt)}>
                        <span class={selectedId === opt.id ? 'selected' : ''}>
                            {`${opt.given_name} ${opt.family_name || ''} <${opt.email}>`}
                        </span>
                    </Button>
                {/each}
            </div>
        </div>

        {#if userStatus}
            <div class="status">
                <h4>{selectedEmail}</h4>

                <div class="font-label stat statLabel">
                    <div>{t.tos.tos}</div>
                    <div>{ta.tos.accepted}</div>
                </div>

                {#each tos as t}
                    {@const stat = userStatus.find(s => s.tos_ts === t.ts)}

                    <div class="stat">
                        <div>
                            {formatDateFromTs(t.ts)}
                        </div>

                        <div>
                            {#if stat}
                                {formatDateFromTs(stat.accept_ts)}
                                -
                                {stat.location}
                            {:else}
                                -
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>
        {/if}

        <Button level={2} onclick={() => closeModalStatus?.()}>
            {t.common.close}
        </Button>
    </Modal>
{/if}

<style>
    .editor {
        margin: 1rem 0;
    }

    .modal {
        width: min(90dvw, 2 * 467pt);
    }

    .optUntil {
        margin: 0.75rem 0;
    }

    .searchOpts {
        min-height: 10rem;
        width: min(30rem, 90dvw);
        margin: 0.5rem 0;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .selected {
        color: hsl(var(--accent));
    }

    .stat {
        display: grid;
        grid-template-columns: 11rem 1fr;
        column-gap: 1rem;
    }

    .status {
        margin-bottom: 1rem;
    }

    .statLabel {
        font-size: 0.9rem;
        color: hsl(var(--text) / 0.7);
    }
</style>
