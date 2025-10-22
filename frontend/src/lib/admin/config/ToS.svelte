<script lang="ts">
    import {useI18nAdmin} from "$state/i18n_admin.svelte.js";
    import {fetchGet, fetchPost} from "$api/fetch.js";
    import {onMount} from "svelte";
    import Button from "$lib/button/Button.svelte";
    import Modal from "$lib/Modal.svelte";
    import EditorText from "$lib/text_edit/EditorText.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {ToSLatestResponse, ToSRequest, ToSUserAcceptResponse} from "$api/types/tos";
    import SearchBar from "$lib/search_bar/SearchBar.svelte";
    import {fetchSearchServer} from "$utils/search";
    import type {UserResponseSimple} from "$api/types/user";

    let t = useI18n();
    let ta = useI18nAdmin();

    let showModalAddNew = $state(false);
    let closeModalAddNew: undefined | (() => void) = $state();
    let showModalStatus = $state(false);
    let closeModalStatus: undefined | (() => void) = $state();
    let isModalOpen = $derived(showModalAddNew || showModalStatus);

    let error = $state('');
    let noneExist = $state(false);
    let tos: undefined | ToSLatestResponse = $state();
    let newToSContent = $state('');

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

    async function searchUser() {
        if (searchValue.length < 3) {
            searchOptions = [];
            return;
        }

        let res = await fetchSearchServer<UserResponseSimple[]>({ty: 'user', idx: 'email', q: searchValue});
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
        let res = await fetchGet<ToSLatestResponse>('/auth/v1/tos/latest');
        if (res.body) {
            tos = res.body;
        } else if (res.status === 204) {
            noneExist = true;
        }
    }

    async function saveToS() {
        error = '';

        let content = newToSContent.trim();
        if (!newToSContent || content.length === 0) {
            return;
        }

        let payload: ToSRequest = {
            is_html: false, // TODO add HTML editor
            content,
        };
        let res = await fetchPost('/auth/v1/tos', payload);
        if (res.error) {
            error = res.error.message;
        } else {
            closeModalAddNew?.();
            await getTos();
        }
    }

    function selectOpt(opt: UserResponseSimple) {
        selectedEmail = opt.email;
        selectedId = opt.id;
    }

</script>

<h2>{ta.tos.tos}</h2>

<p>
    {#if noneExist}
        {ta.tos.noneExist}
    {:else if tos}
        {tos.content}
    {/if}
</p>

<div class="flex gap-05 flex-wrap">
    <Button
            ariaLabel={ta.tos.addNewToS}
            level={isModalOpen ? 2 : 1}
            onclick={() => showModalAddNew = true}
    >
        {ta.tos.addNewToS}
    </Button>
    {#if tos}
        <Button
                ariaLabel={ta.tos.checkStatus}
                level={isModalOpen ? 3 : 2}
                onclick={() => showModalStatus = true}
        >
            {ta.tos.checkStatus}
        </Button>
    {/if}
</div>

<Modal bind:showModal={showModalAddNew} bind:closeModal={closeModalAddNew}>
    <div class="modal">
        <h3>{ta.tos.addNewToS}</h3>

        <div class="editor">
            <EditorText
                    bind:content={newToSContent}
                    height="min(80dvh, 40rem)"
                    hideButtons
            />
        </div>

        <div class="action" style:margin-bottom="1rem">
            <b>{ta.tos.immutable}</b>
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

{#if tos}
    <Modal bind:showModal={showModalStatus} bind:closeModal={closeModalStatus}>
        <h3>{ta.tos.checkStatus}</h3>

        <div class="userSearch">
            <SearchBar placeholder="E-Mail" bind:value={searchValue}/>
            <div class="searchOpts">
                {#each searchOptions as opt}
                    <Button level={3} onclick={() => selectOpt(opt)}>
                        {`${opt.given_name} ${opt.family_name || ''} <${opt.email}>`}
                    </Button>
                {/each}
            </div>
        </div>

        {#if userStatus && userStatus.length > 0}
            <div class="status">
                <h4>{selectedEmail}</h4>

                {#each userStatus as stat}
                    <div class="stat">
                        {stat.tos_ts}
                        {stat.location}
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
        width: min(90dvw, 40rem);
    }

    .searchOpts {
        min-height: 10rem;
        width: min(30rem, 90dvw);
        margin: .5rem 0;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
</style>
