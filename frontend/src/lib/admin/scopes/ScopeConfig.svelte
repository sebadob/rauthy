<script lang="ts">
    import { isDefaultScope } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import Input from '#lib/form/Input.svelte';
    import type { UserAttrConfigValueResponse } from '#api/types/user_attrs.ts';
    import type { ScopeRequest, ScopeResponse } from '#api/types/scopes.ts';
    import IconCheck from '#icons/IconCheck.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { fetchPut } from '#api/fetch.ts';
    import Form from '#lib/form/Form.svelte';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import Section from '#lib/Section.svelte';
    import type { SelectItem } from '#lib/select_list/props.ts';
    import SelectList from '#lib/select_list/SelectList.svelte';
    import { PATTERN_ROLE_SCOPE } from '#utils/patterns.ts';
    import { untrack } from 'svelte';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';

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

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);
    let isDefault = $derived(isDefaultScope(scope.name));

    let name = $state(untrack(() => scope.name));
    let itemsAccess: undefined | SelectItem[] = $state();
    let itemsId: undefined | SelectItem[] = $state();
    let claimsAtRoot = $state(untrack(() => scope.claims_at_root));

    $effect(() => {
        if (scope.id) {
            name = scope.name;
            claimsAtRoot = scope.claims_at_root;
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

        if (isDefaultScope(name) || (scope.name !== name && scopes.find(s => s.name === name))) {
            err = ta.common.nameExistsAlready;
            return;
        }

        let payload: ScopeRequest = {
            scope: name,
        };
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
        payload.claims_at_root = claimsAtRoot;

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            onSave();
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }
</script>

<Form action={`/auth/v1/scopes/${scope.id}`} {onSubmit}>
    <div class="ui-section-stack">
        <Section title={ta.common.information}>
            <div class="ui-field-grid">
                <LabeledValue label="ID" mono copyToClip={scope.id}>
                    {scope.id}
                </LabeledValue>

                <Input
                    bind:value={name}
                    autocomplete="off"
                    label={ta.scopes.name}
                    placeholder={ta.scopes.name}
                    disabled={isDefault}
                    width="100%"
                    required
                    pattern={PATTERN_ROLE_SCOPE}
                />
            </div>
            {#if isDefault}<p class="ui-muted-copy">{ta.scopes.defaultNoMod}</p>{/if}
        </Section>
        {#if !isDefault}
            <Section>
                <p class="ui-muted-copy">{ta.scopes.mapping1}</p>
                <p class="ui-muted-copy">{ta.scopes.mapping2}</p>
                <div class="ui-field-grid">
                    {#if itemsAccess}
                        <SelectList maxWidth="100%" bind:items={itemsAccess}
                            >Access Token Mappings</SelectList
                        >
                    {/if}

                    {#if itemsId}
                        <SelectList maxWidth="100%" bind:items={itemsId}
                            >Id Token Mappings</SelectList
                        >
                    {/if}
                </div>
                <div class="rootClaims">
                    <InputCheckbox bind:checked={claimsAtRoot} ariaLabel={ta.scopes.claimsAtRoot}>
                        {ta.scopes.claimsAtRoot}
                    </InputCheckbox>
                </div>
                {#if claimsAtRoot}
                    <p class="warn">
                        {ta.scopes.claimsAtRootWarning}
                        <a
                            href="https://www.iana.org/assignments/jwt/jwt.xhtml"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            IANA JWT Claims Registry
                        </a>
                    </p>
                {/if}
            </Section>
        {/if}
    </div>

    {#if !isDefault}
        <div class="flex gap-05 actions">
            <Button type="submit">
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck />
            {/if}
        </div>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    {/if}
</Form>

<style>
    .rootClaims {
        padding-block-start: 0.75rem;
        border-top: 1px solid var(--ui-outline);
    }

    .actions {
        margin-block: var(--ui-space);
    }

    .ui-muted-copy,
    .warn {
        margin: 0;
    }

    .warn {
        max-width: 30rem;
        color: hsl(var(--error));
        font-size: 0.9rem;
        opacity: 0.8;
    }
</style>
