<script lang="ts">
    import {onMount} from "svelte";
    import {useIsDev} from "$state/is_dev.svelte.ts";
    import {fetchGet} from "$api/fetch.ts";

    let {
        id,
        data = $bindable(),
    }: {
        id: string,
        data: boolean | string | object,
    } = $props();

    onMount(async () => {
        if (!useIsDev()) {
            let tpl = document.getElementById(id);
            if (tpl) {
                assign(tpl.innerHTML);
            }
        } else {
            let res = await fetchGet<typeof data>(`/auth/v1/template/${id}`);
            if (res.error) {
                console.error(res.error);
            } else if (res.text) {
                assign(res.text);
            }
        }
    });

    function assign(s: string) {
        if (typeof data === 'boolean') {
            data = s === 'true';
        } else if (typeof data === 'string') {
            data = s;
        } else {
            data = JSON.parse(s);
        }
    }
</script>
