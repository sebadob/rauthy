<script lang="ts">
    import {isDefaultScope} from "$utils/helpers.ts";
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import type {UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import type {ScopeRequest, ScopeResponse} from "$api/types/scopes.ts";
    import IconCheck from "$icons/IconCheck.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {fetchPut} from "$api/fetch.ts";
    import Form from "$lib5/form/Form.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import type {SelectItem} from "$lib5/select_list/props.ts";
    import SelectList from "$lib5/select_list/SelectList.svelte";

    let {
        attrs,
        scope,
        scopes,
        onSave,
    }: {
        attrs: UserAttrConfigValueResponse[],
        scope: ScopeResponse,
        scopes: ScopeResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);
    let isDefault = $derived(isDefaultScope(scope.name));

    let name = $state(scope.name);
    let itemsAccess: undefined | SelectItem[] = $state();
    let itemsId: undefined | SelectItem[] = $state();

    $effect(() => {
        if (scope.id) {
            name = scope.name;
        }
    });

    $effect(() => {
        if (isDefaultScope(scope.name)) {
            itemsAccess = undefined;
            itemsId = undefined;
        } else {
            itemsAccess = attrs
                .map(a => {
                    let i: SelectItem = {
                        name: a.name,
                        selected: scope.attr_include_access?.includes(a.name) || false,
                    };
                    return i;
                })
                .toSorted((a, b) => a.name.localeCompare(b.name));
            itemsId = attrs
                .map(a => {
                    let i: SelectItem = {
                        name: a.name,
                        selected: scope.attr_include_id?.includes(a.name) || false,
                    };
                    return i;
                })
                .toSorted((a, b) => a.name.localeCompare(b.name));
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (isDefaultScope(name) || scopes.find(s => s.name === name)) {
            err = ta.common.nameExistsAlready;
            return;
        }

        let payload: ScopeRequest = {
            scope: name,
        }
        if (itemsAccess) {
            let filtered = itemsAccess.filter(i => i.selected).map(i => i.name);
            if (filtered.length > 0) {
                payload.attr_include_access = filtered;
            }
        }
        if (itemsId) {
            let filtered = itemsId.filter(i => i.selected).map(i => i.name);
            if (filtered.length > 0) {
                payload.attr_include_id = filtered;
            }
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

        {#if itemsAccess}
            <SelectList bind:items={itemsAccess}>
                Access Token Mappings
            </SelectList>
        {/if}

        {#if itemsId}
            <SelectList bind:items={itemsId}>
                Id Token Mappings
            </SelectList>
        {/if}
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
