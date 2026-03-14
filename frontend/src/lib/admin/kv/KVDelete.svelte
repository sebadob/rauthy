<script lang="ts">
    import type { IParam } from '$state/param.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { fetchDelete } from '$api/fetch';

    let {
        ns,
        onDelete,
    }: {
        ns: IParam;
        onDelete: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    async function deleteNs() {
        let res = await fetchDelete(`/auth/v1/kv/ns/${ns.get()}`);
        if (res.error) {
            console.error(res.error.message);
        } else {
            onDelete();
        }
    }
</script>

<p>{ta.kv.delNsMsg}</p>

<Button level={-1} onclick={deleteNs}>
    {t.common.delete}
</Button>
