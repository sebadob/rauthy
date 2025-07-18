<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import Options from "$lib5/Options.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import {fetchPut} from "$api/fetch";
    import Form from "$lib5/form/Form.svelte";
    import type {UserAttrConfigRequest, UserAttrConfigTyp, UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import {PATTERN_ATTR, PATTERN_ATTR_DESC} from "$utils/patterns";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {slide} from "svelte/transition";
    import { ATTR_TYPES } from "$utils/constants";

    let {
        attr,
        attrs,
        onSave,
    }: {
        attr: UserAttrConfigValueResponse,
        attrs: UserAttrConfigValueResponse[],
        onSave: () => void,
    } = $props();

    const width = "20rem";

    let refSubmit: undefined | HTMLButtonElement = $state();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);

    let name = $state(attr.name);
    let desc = $state(attr.desc);
    let defaultValue = $state(attr.default_value);
    let typ: UserAttrConfigTyp | '-'  = $state(attr.typ || '-')
    let userEditable = $state(attr.user_editable || false);


    let showMakeEditable = $state(false);

    $effect(() => {
        if (attr.name) {
            name = attr.name;
            desc = attr.desc;
            defaultValue = attr.default_value;
            typ = attr.typ || '-';
            userEditable = attr.user_editable || false;

            showMakeEditable = false;
        }
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
            default_value: defaultValue || undefined,
            typ: typ != '-' ? typ : undefined,
            user_editable: userEditable || false,
        }

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
            autocomplete="off"
            label={ta.attrs.defaultValue}
            placeholder={ta.attrs.defaultValue}
            {width}
    />
    <div class="flex gap-05">
        {ta.attrs.typ}
        <Options
            ariaLabel={ta.attrs.typ}
            options={ATTR_TYPES}
            bind:value={typ}
        />
    </div>

    <div class="editableRow">
        <div class="flex gap-05">
            {ta.attrs.userEditable}
            <CheckIcon checked={userEditable}/>
        </div>

        {#if !userEditable}
            <div class="editable">
                <Button
                        ariaLabel={ta.attrs.makeEditable}
                        level={showMakeEditable ? 3 : 2}
                        onclick={() => showMakeEditable = !showMakeEditable}
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
            <IconCheck/>
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
        gap: .5rem;
        align-items: center;
    }

    .editable {
        margin-top: .5rem;
    }

    .editableRow {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
    }

    .editableRow > .editable {
        margin: .2rem 0 0 .5rem;
    }
</style>
