<script lang="ts">
    import {isDefaultScope} from "$utils/helpers.ts";
    import Button from "$lib5/button/Button.svelte";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import Input from "$lib5/form/Input.svelte";
    import type {UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import type {ScopeRequest, ScopeResponse} from "$api/types/scopes.ts";
    import IconCheck from "$icons/IconCheck.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {fetchPut} from "$api/fetch.ts";
    import Form from "$lib5/form/Form.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";

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

    let err = $state('');
    let success = $state(false);
    let isDefault = $derived(isDefaultScope(scope.name));
    let allAttrs: string[] = $state([]);

    let name = $state(scope.name);

    $effect(() => {
        if (scope.id) {
            name = scope.name;
        }
    });

    $effect(() => {
        if (attrs) {
            allAttrs = attrs.map(a => a.name);
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        let payload: ScopeRequest = {
            scope: name,
        }
        if (scope.attr_include_access && scope.attr_include_access.length > 0) {
            payload.attr_include_access = scope.attr_include_access;
        }
        if (scope.attr_include_id && scope.attr_include_id.length > 0) {
            payload.attr_include_id = scope.attr_include_id;
        }

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
                onSave();
            }, 2000);
        }
    }
</script>

<Form action={`/auth/v1/scopes/${scope.id}`} {onSubmit}>
    <LabeledValue label="ID" mono>
        {scope.id}
    </LabeledValue>

    <Input
            bind:value={name}
            autocomplete="off"
            label={ta.scopes.name}
            placeholder={ta.scopes.name}
            disabled={isDefault}
            width="14.5rem"
    />

    {#if isDefault}
        <p>{ta.scopes.defaultNoMod}</p>
    {:else}
        <p>{ta.scopes.mapping1}</p>
        <p>{ta.scopes.mapping2}</p>

        <!-- TODO Access Mappings -->
        <div>
            <div class="label">
                Access Token Mappings
            </div>
            <ItemTiles
                    options={allAttrs}
                    bind:items={scope.attr_include_access}
                    searchThreshold={4}
            />
        </div>

        <!-- TODO ID Mappings -->
        <div>
            <div class="label">
                ID Token Mappings
            </div>
            <ItemTiles
                    options={allAttrs}
                    bind:items={scope.attr_include_id}
                    searchThreshold={4}
            />
        </div>
    {/if}

    {#if !isDefault}
        <Button type="submit">
            {t.common.save}
        </Button>

        {#if success}
            <IconCheck/>
        {/if}

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    {/if}
</Form>

<style>
</style>
