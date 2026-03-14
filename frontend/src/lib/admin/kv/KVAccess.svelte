<script lang="ts">
    import type { IParam } from '$state/param.svelte';
    import { fetchGet } from '$api/fetch';
    import type { KVAccessResponse } from '$api/types/kv';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';

    let {
        ns,
    }: {
        ns: IParam;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let accessKeys: KVAccessResponse[] = $state([]);

    let urlAccess = $derived(`/auth/v1/kv/ns/${ns.get()}/access`);

    $effect(() => {
        fetchData();
    });

    async function fetchData() {
        let res = await fetchGet<KVAccessResponse[]>(urlAccess);
        if (res.body) {
            accessKeys = res.body;
        } else {
            err = res.error?.message || 'Fetch error';
        }
    }
</script>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

{#if accessKeys.length === 0}
    {ta.common.noEntries}
{:else}
    {#each accessKeys as access (access.id)}
        {access.id}
        {access.name}
    {/each}
{/if}
