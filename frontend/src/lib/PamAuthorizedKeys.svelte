<script lang="ts">
    import type { PamSshAuthKeyRequest, PamSshAuthKeyResponse } from '$api/types/pam';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Modal from '$lib/Modal.svelte';
    import InputArea from '$lib/form/InputArea.svelte';
    import Form from '$lib/form/Form.svelte';
    import { fetchDelete, fetchPost } from '$api/fetch';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import { formatDateFromTs } from '$utils/helpers';
    import IconTrash from '$icons/IconTrash.svelte';
    import Tooltip from '$lib/Tooltip.svelte';

    let {
        authorizedKeys,
        onSave,
    }: {
        authorizedKeys: undefined | PamSshAuthKeyResponse[];
        onSave: () => void;
    } = $props();

    const now = new Date().getTime() / 1000;

    let t = useI18n();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let error = $state('');

    $effect(() => {
        if (error) {
            setTimeout(() => {
                error = '';
            }, 5000);
        }
    });

    async function deleteKey(tsAdded: number) {
        let res = await fetchDelete(`/auth/v1/pam/users/self/authorized_keys/${tsAdded}`);
        if (res.error) {
            error = res.error.message;
        } else {
            onSave();
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        let payload: PamSshAuthKeyRequest = {
            data: params.get('key')?.trim() || '',
        };

        let res = await fetchPost(form.action, payload);
        if (res.error) {
            error = res.error.message;
        } else {
            onSave();
            closeModal?.();
        }
    }
</script>

{#if authorizedKeys}
    <div class="container">
        <div class="keyAdd">
            <h4>SSH Keys</h4>
            <Button level={2} onclick={() => (showModal = true)}>
                {t.account.pam.addSshKey}
            </Button>
        </div>

        <Modal bind:showModal bind:closeModal>
            <h3>{t.account.pam.addSshKey}</h3>
            <Form action="/auth/v1/pam/users/self/authorized_keys" {onSubmit}>
                <InputArea
                    name="key"
                    label={t.account.pam.addSshKey}
                    placeholder="ssh-... <key> <comment>"
                    rows={10}
                    maxLength={8192}
                    required
                    fontMono
                    width="min(60rem, 90dvw)"
                />

                <Button type="submit">{t.common.save}</Button>
                {#if error}
                    <div class="err">
                        {error}
                    </div>
                {/if}
            </Form>
        </Modal>

        {#each authorizedKeys as key}
            <div class="key">
                <div class="keyInfo">
                    <div>
                        <LabeledValue label={t.account.pam.keyAdded}>
                            {formatDateFromTs(key.ts_added)}
                        </LabeledValue>
                        {#if key.expires}
                            <LabeledValue label={t.account.accessExp}>
                                <div class={now > key.expires ? 'expired' : ''}>
                                    {formatDateFromTs(key.expires)}
                                </div>
                            </LabeledValue>
                        {/if}
                        <LabeledValue label={t.account.pam.keyType}>
                            {key.typ}
                        </LabeledValue>
                        <LabeledValue label={t.account.pam.comment}>
                            {key.comment}
                        </LabeledValue>
                    </div>
                    <div class="del">
                        <Tooltip text={t.common.delete} xOffset={-64}>
                            <Button onclick={() => deleteKey(key.ts_added)} invisible>
                                <IconTrash />
                            </Button>
                        </Tooltip>
                    </div>
                </div>
                <LabeledValue label="Key">
                    <div class="font-mono">
                        {key.data}
                    </div>
                </LabeledValue>
            </div>
        {/each}
    </div>
{/if}

<style>
    .container {
        margin: 1rem 0;
    }

    .del {
        margin: 0.75rem 0.5rem;
    }

    .err {
        margin-top: 0.5rem;
        max-width: min(40rem, 90dvw);
    }

    .expired {
        color: hsl(var(--error));
        font-weight: bold;
    }

    .key {
        margin: 0.5rem 0;
        padding: 0.25rem 0.5rem;
        background: hsla(var(--bg-high) / 0.2);
        border-radius: var(--border-radius);
        word-break: break-all;
    }

    .keyAdd {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .keyInfo {
        display: flex;
        justify-content: space-between;
    }
</style>
