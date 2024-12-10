<script>
    import {onMount} from "svelte";
    import {getQueryParams} from "../../utils/helpers";
    import TabBar from "$lib/TabBar.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} t
     * @property {any} [selected]
     * @property {boolean} [showWebId]
     */

    /** @type {Props} */
    let { t, selected = $bindable(t.navInfo), showWebId = false } = $props();

    let labels = $state(showWebId ?
        [t.navInfo, t.navEdit, t.navPassword, t.navMfa, 'WebID', t.devices, t.navLogout]
        : [t.navInfo, t.navEdit, t.navPassword, t.navMfa, t.devices, t.navLogout]);

    onMount(() => {
        let params = getQueryParams();
        if (params.v === 'devices') {
            selected = t.devices;
        }
    });

</script>

<TabBar bind:labels bind:selected/>
