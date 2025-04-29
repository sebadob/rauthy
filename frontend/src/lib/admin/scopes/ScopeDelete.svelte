<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {isDefaultScope} from "$utils/helpers";
    import type {ScopeResponse} from "$api/types/scopes.ts";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import {fetchDelete} from "$api/fetch";

    let {
        scope,
        onSave,
    }: {
        scope: ScopeResponse,
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');

    async function onSubmit() {
        err = '';

        let res = await fetchDelete(`/auth/v1/scopes/${scope.id}`);
        if (res.error) {
            err = res.error.message;
        } else {
            onSave();
        }
    }
</script>

{#if isDefaultScope(scope.name)}
    <p>{ta.scopes.deleteDefault}</p>
{:else}
    <p>{ta.scopes.delete1}</p>

    <Button level={-1} onclick={onSubmit}>
        {t.common.delete}
    </Button>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
{/if}
