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

    let err = $state('');
    let success = $state(false);
    let name = $state(attr.name);
    let desc = $state(attr.desc);

    $effect(() => {
        if (attr.name) {
            name = attr.name;
            desc = attr.desc;
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
            width="14.5rem"
            required
            pattern={PATTERN_ATTR}
    />
    <Input
            bind:value={desc}
            autocomplete="off"
            label={ta.attrs.desc}
            placeholder={ta.attrs.desc}
            width="14.5rem"
            pattern={PATTERN_ATTR_DESC}
    />

    <div class="flex gap-05">
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
</style>
