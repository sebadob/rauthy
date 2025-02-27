<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import type {ApiKeyResponse} from "$api/types/api_keys.ts";
    import {fetchDelete} from "$api/fetch.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

    let {
        key,
        onSave,
    }: {
        key: ApiKeyResponse,
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');

    async function onSubmit() {
        err = '';

        let res = await fetchDelete(`/auth/v1/api_keys/${key.name}`);
        if (res.error) {
            err = res.error.message;
        } else {
            onSave();
        }
    }

</script>

<p>{ta.api_key.delete1}</p>

<Button level={-1} onclick={onSubmit}>
    {t.common.delete}
</Button>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}
