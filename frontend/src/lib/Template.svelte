<script lang="ts">
    import {onMount} from "svelte";
    import {useIsDev} from "$state/is_dev.svelte";
    import {fetchGet} from "$api/fetch";

    type T = $$Generic;

    let {
        id,
        value = $bindable(),
    }: {
        id: string,
        value: boolean | string | number | T,
    } = $props();

    onMount(async () => {
        if (!useIsDev().get()) {
            let tpl = document.getElementById(id);
            if (tpl) {
                assign(tpl.innerHTML);
            }
        } else {
            let res = await fetchGet<typeof value>(`/auth/v1/template/${id}`);
            if (res.error) {
                console.error(res.error);
            } else if (res.text) {
                assign(res.text);
            }
        }
    });

    function assign(s: string) {
        if (typeof value === 'boolean') {
            value = s === 'true';
        } else if (typeof value === 'string') {
            value = s;
        } else if (typeof value === 'number') {
            value = Number.parseInt(s);
        } else {
            value = JSON.parse(s);
        }
    }
</script>
