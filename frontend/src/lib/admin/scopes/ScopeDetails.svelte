<script lang="ts">
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import { isDefaultScope } from '#utils/helpers.ts';
    import IconId from '#icons/IconId.svelte';
    import ScopeConfig from './ScopeConfig.svelte';
    import type { UserAttrConfigValueResponse } from '#api/types/user_attrs.ts';
    import type { ScopeResponse } from '#api/types/scopes.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        attrs,
        scope,
        scopes,
        onSave,
    }: {
        attrs: UserAttrConfigValueResponse[];
        scope: ScopeResponse;
        scopes: ScopeResponse[];
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();
</script>

<EntityHeader title={scope.name} subtitle={ta.nav.scopes}>
    {#snippet actions()}
        {#key scope.id}
            <EntityDelete
                url={`/auth/v1/scopes/${scope.id}`}
                name={scope.name}
                message={ta.scopes.delete1}
                disabledReason={isDefaultScope(scope.name) ? ta.scopes.deleteDefault : undefined}
                onDelete={onSave}
            />
        {/key}
    {/snippet}
    <IconId width="28px" />
</EntityHeader>

<ScopeConfig {attrs} {scope} {scopes} {onSave} />
