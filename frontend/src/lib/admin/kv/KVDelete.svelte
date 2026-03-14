<script lang="ts">
    import type { IParam } from '$state/param.svelte';

    let {
        ns,
        onSave,
    }: {
        ns: IParam;
        onSave: () => void;
    } = $props();

    let url = $derived(`/auth/v1/kv/ns/${ns.get()}`);
</script>

TODO DELETE
