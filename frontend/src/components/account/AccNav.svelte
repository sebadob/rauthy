<script>
    import {onMount} from "svelte";
    import TabBar from "$lib/TabBar.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useParam} from "$state/param.svelte.js";

    /**
     * @typedef {Object} Props
     * @property {any} [selected]
     * @property {boolean} [showWebId]
     */

    /** @type {Props} */
    let {selected = $bindable(), showWebId = false} = $props();

    let t = useI18n();

    let labels = $state(showWebId ?
        [
            t.account.navInfo,
            t.account.navEdit,
            t.common.password,
            t.account.navMfa, 'WebID',
            t.account.devices,
            t.account.navLogout,
        ]
        : [
            t.account.navInfo,
            t.account.navEdit,
            t.common.password,
            t.account.navMfa,
            t.account.devices,
            t.account.navLogout,
        ]);

    onMount(() => {
        if (useParam('v').get() === 'devices') {
            selected = t.account.devices;
        }
    });

</script>

<TabBar {labels} bind:selected/>
