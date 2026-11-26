<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import Input from '#lib/form/Input.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { fetchPut } from '#api/fetch.ts';
    import Form from '#lib/form/Form.svelte';
    import type {
        UserAttrConfigRequest,
        UserAttrConfigValueResponse,
    } from '#api/types/user_attrs.ts';
    import { PATTERN_ATTR, PATTERN_ATTR_DESC } from '#utils/patterns.ts';
    import CheckIcon from '#lib/CheckIcon.svelte';
    import { slide } from 'svelte/transition';
    import { untrack } from 'svelte';
    import { parseJsonValue, stringifyJsonValue } from '#utils/jsonValue.ts';
    import { type UnsavedField } from '#utils/unsaved.ts';

    let {
        attr,
        attrs,
        fields = $bindable([]),
        reset = $bindable(() => {}),
        onSave,
    }: {
        attr: UserAttrConfigValueResponse;
        attrs: UserAttrConfigValueResponse[];
        // the parent owns the `<UnsavedGuard>`, so it gets the watched values
        fields?: UnsavedField[];
        // the parent's guard calls this to drop local edits
        reset?: () => void;
        onSave: () => void;
    } = $props();

    const width = '20rem';

    let refSubmit: undefined | HTMLButtonElement = $state();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);

    let name = $state(untrack(() => attr.name));
    let desc = $state(untrack(() => attr.desc));
    let defaultValue = $state(untrack(() => stringifyJsonValue(attr.default_value, 1)));
    let userEditable = $state(untrack(() => attr.user_editable || false));

    let showMakeEditable = $state(false);

    function resetForm() {
        name = attr.name;
        desc = attr.desc;
        defaultValue = stringifyJsonValue(attr.default_value, 1);
        userEditable = attr.user_editable || false;
        showMakeEditable = false;
    }

    $effect(() => {
        if (attr.name) {
            resetForm();
        }
    });

    let watched = $derived.by(() => [
        { value: name, original: attr.name },
        { value: desc, original: attr.desc },
        { value: defaultValue, original: stringifyJsonValue(attr.default_value, 1) },
        { value: userEditable, original: attr.user_editable || false },
    ]);

    // the parent owns navigation guards, so it needs to know about the form state
    $effect(() => {
        fields = watched;
        reset = resetForm;
    });

    async function submitMakeEditable() {
        userEditable = true;
        refSubmit?.click();
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (name !== attr.name && attrs.find(a => a.name === name)) {
            err = ta.common.nameExistsAlready;
            return;
        }

        let payload: UserAttrConfigRequest = {
            name,
            desc: desc || undefined,
            default_value: parseJsonValue(defaultValue),
            user_editable: userEditable || false,
        };

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

<Form action={`/auth/v1/users/attr/${attr.name}`} {onSubmit}>
    <Input
        bind:value={name}
        autocomplete="off"
        label={ta.attrs.name}
        placeholder={ta.attrs.name}
        required
        pattern={PATTERN_ATTR}
        {width}
    />
    <Input
        bind:value={desc}
        autocomplete="off"
        label={ta.attrs.desc}
        placeholder={ta.attrs.desc}
        pattern={PATTERN_ATTR_DESC}
        {width}
    />
    <Input
        bind:value={defaultValue}
        formatBadge={t.account.stringOrJson ?? 'String / JSON'}
        autocomplete="off"
        label={ta.attrs.defaultValue}
        placeholder={ta.attrs.defaultValue}
        {width}
    />

    <div class="editableRow">
        <div class="flex gap-05">
            {ta.attrs.userEditable}
            <CheckIcon checked={userEditable} />
        </div>

        {#if !userEditable}
            <div class="editable">
                <Button
                    ariaLabel={ta.attrs.makeEditable}
                    level={showMakeEditable ? 3 : 2}
                    onclick={() => (showMakeEditable = !showMakeEditable)}
                >
                    {ta.attrs.makeEditable}
                </Button>
            </div>
        {/if}
    </div>

    {#if !userEditable}
        <div class="editable">
            {#if showMakeEditable}
                <div transition:slide={{ duration: 150 }}>
                    <p>{ta.attrs.makeEditableP1}</p>
                    <p class="err">{@html ta.attrs.makeEditableP2}</p>
                    <p class="err">{ta.attrs.makeEditableP3}</p>

                    <Button
                        ariaLabel={ta.attrs.makeEditable}
                        level={-1}
                        onclick={submitMakeEditable}
                    >
                        {ta.attrs.makeEditable}
                    </Button>
                </div>
            {/if}
        </div>
    {/if}

    <div class="btn">
        <Button type="submit" level={showMakeEditable ? 2 : 1} bind:ref={refSubmit}>
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
</Form>

<style>
    .btn {
        margin-top: 1rem;
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }

    .editable {
        margin-top: 0.5rem;
    }

    .editableRow {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
    }

    .editableRow > .editable {
        margin: 0.2rem 0 0 0.5rem;
    }
</style>
