<script lang="ts">
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import IconDocText from '#icons/IconDocText.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import type { UserAttrConfigValueResponse } from '#api/types/user_attrs.ts';
    import AttrConfig from '#lib/admin/attrs/AttrConfig.svelte';

    let {
        attr,
        attrs,
        onSave,
    }: {
        attr: UserAttrConfigValueResponse;
        attrs: UserAttrConfigValueResponse[];
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();
</script>

<EntityHeader title={attr.name} subtitle={ta.nav.attributes}>
    {#snippet actions()}
        {#key attr.name}
            <EntityDelete
                url={`/auth/v1/users/attr/${attr.name}`}
                name={attr.name}
                message={ta.attrs.delete1}
                onDelete={onSave}
            />
        {/key}
    {/snippet}
    <IconDocText width="28px" />
</EntityHeader>

<AttrConfig {attr} {attrs} {onSave} />
