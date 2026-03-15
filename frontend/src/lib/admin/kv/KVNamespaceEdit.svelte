<script lang="ts">
    import type { IParam } from '$state/param.svelte';
    import Form from '$lib/form/Form.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Input from '$lib/form/Input.svelte';
    import { PATTERN_GROUP } from '$utils/patterns';
    import { fetchGet, fetchPut } from '$api/fetch';
    import type { KVNamespaceRequest, KVNamespaceResponse } from '$api/types/kv';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import IconCheck from '$icons/IconCheck.svelte';

    let {
        ns,
        onSave,
    }: {
        ns: undefined | KVNamespaceResponse;
        onSave: (nameNew: string) => void;
    } = $props();

    let t = useI18n();

    interface Namespace {
        name: string;
        public: boolean;
    }

    let namespace: undefined | Namespace = $state();

    let success = $state(false);
    let url = $derived(`/auth/v1/kv/ns/${ns?.name}`);

    $effect(() => {
        if (!ns) {
            return;
        }
        namespace = {
            name: ns.name,
            public: ns.public || false,
        };
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (!namespace || !ns) {
            return;
        }

        let payload: KVNamespaceRequest = {
            name: namespace.name,
            public: namespace.public || undefined,
        };
        let res = await fetchPut(url, payload);
        if (res.error) {
            console.error(res.error);
        } else {
            onSave(payload.name);
            success = true;
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }
</script>

<div class="container">
    {#if namespace}
        <Form action={url} {onSubmit}>
            <Input pattern={PATTERN_GROUP} bind:value={namespace.name} />
            <InputCheckbox ariaLabel="Public Access" bind:checked={namespace.public}>
                Public Access
            </InputCheckbox>

            <div class="btn">
                <Button type="submit">
                    {t.common.save}
                </Button>
                {#if success}
                    <IconCheck />
                {/if}
            </div>
        </Form>
    {/if}
</div>

<style>
    .btn {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .container {
        margin-top: 1rem;
    }
</style>
