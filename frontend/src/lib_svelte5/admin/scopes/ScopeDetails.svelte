<script lang="ts">
    import ScopeConfig from "./ScopeConfig.svelte";
    import ScopeDelete from "../../../components/admin/scopes/ScopeDelete.svelte";
    import type {UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import type {ScopeResponse} from "$api/types/scopes.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";

    let {
        attrs,
        scope,
        onSave,
    }: {
        attrs: UserAttrConfigValueResponse[],
        scope: ScopeResponse,
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const tabs = [
        ta.nav.config,
        t.common.delete,
    ];
    let selected = $state(tabs[0]);

</script>

<div class="flex">
    <Tabs {tabs} bind:selected/>
</div>

{#if selected === ta.nav.config}
    <ScopeConfig {attrs} {scope} {onSave}/>
{:else if selected === t.common.delete}
    <ScopeDelete {scope} {onSave}/>
{/if}
