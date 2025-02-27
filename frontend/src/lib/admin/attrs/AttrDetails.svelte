<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import AttrConfig from "$lib5/admin/attrs/AttrConfig.svelte";
    import AttrDelete from "$lib5/admin/attrs/AttrDelete.svelte";

    let {
        attr,
        attrs,
        onSave,
    }: {
        attr: UserAttrConfigValueResponse,
        attrs: UserAttrConfigValueResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const tabs = [
        ta.nav.config,
        t.common.delete,
    ];
    let selected = $state(tabs[0]);

    let focusFirst: undefined | (() => void) = $state();

    $effect(() => {
        if (attr.name) {
            requestAnimationFrame(() => {
                focusFirst?.();
            });
        }
    });

</script>

<div class="flex">
    <Tabs {tabs} bind:selected bind:focusFirst/>
</div>

{#if selected === ta.nav.config}
    <AttrConfig {attr} {attrs} {onSave}/>
{:else if selected === t.common.delete}
    <AttrDelete {attr} {onSave}/>
{/if}
