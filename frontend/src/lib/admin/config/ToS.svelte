<script lang="ts">
    import {useI18nAdmin} from "$state/i18n_admin.svelte.js";
    import {fetchGet, fetchPost} from "$api/fetch.js";
    import {onMount} from "svelte";
    import Button from "$lib/button/Button.svelte";
    import Modal from "$lib/Modal.svelte";
    import EditorText from "$lib/text_edit/EditorText.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {ToSLatestResponse, ToSRequest} from "$api/types/tos";

    let t = useI18n();
    let ta = useI18nAdmin();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let error = $state('');
    let noneExist = $state(false);
    let tos: undefined | ToSLatestResponse = $state();
    let newToSContent = $state('');

    onMount(() => {
        getTos();
    });

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
            closeModal?.();
            await getTos();
        }
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

<Button level={showModal ? 2 : 1} ariaLabel={ta.tos.addNewToS} onclick={() => showModal = true}>
    {ta.tos.addNewToS}
</Button>

<Modal bind:showModal bind:closeModal>
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
        <Button level={-3} ariaLabel={t.common.cancel} onclick={() => closeModal?.()}>
            {t.common.cancel}
        </Button>

        {#if error}
            <div class="err">
                {error}
            </div>
        {/if}
    </div>
</Modal>

<style>
    .editor {
        margin: 1rem 0;
    }

    .modal {
        width: min(90dvw, 40rem);
    }
</style>
