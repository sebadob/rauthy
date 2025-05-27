<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import {fetchPut} from "$api/fetch";
    import Form from "$lib5/form/Form.svelte";
    import type {UserAttrConfigRequest, UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import {PATTERN_ATTR, PATTERN_ATTR_DESC} from "$utils/patterns";
    import InputCheckbox from "$lib/form/InputCheckbox.svelte";

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

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);

    let name = $state(attr.name);
    let desc = $state(attr.desc);
    let defaultValue = $state(attr.default_value);
    let userEditable = $state(attr.user_editable || false);

    $effect(() => {
        if (attr.name) {
            name = attr.name;
            desc = attr.desc;
            defaultValue = attr.default_value;
            userEditable = attr.user_editable || false;
        }
    });

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
            placeholder="JSON Value"
            {width}
    />
    <!--    TODO this is not yet respected in the backend, but CRUD works-->
    <!--    <InputCheckbox-->
    <!--            ariaLabel={ta.attrs.userEditable}-->
    <!--            bind:checked={userEditable}-->
    <!--    >-->
    <!--        {ta.attrs.userEditable}-->
    <!--    </InputCheckbox>-->

    <div class="btn">
        <Button type="submit">
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
</style>
