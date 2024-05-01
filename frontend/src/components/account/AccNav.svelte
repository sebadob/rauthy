<script>
    import {onMount} from "svelte";
    import {getQueryParams} from "../../utils/helpers";
    import TabBar from "$lib/TabBar.svelte";

    export let t;
    export let selected = t.navInfo;
    export let showWebId = false;

    let labels = showWebId ?
        [t.navInfo, t.navEdit, t.navPassword, t.navMfa, 'WebID', t.devices, t.navLogout]
        : [t.navInfo, t.navEdit, t.navPassword, t.navMfa, t.devices, t.navLogout];

    onMount(() => {
        let params = getQueryParams();
        if (params.v === 'devices') {
            selected = t.devices;
        }
    });

</script>

<TabBar bind:labels bind:selected/>
