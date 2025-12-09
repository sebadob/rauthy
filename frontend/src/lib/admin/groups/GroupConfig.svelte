<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { fetchPut } from '$api/fetch';
    import Form from '$lib5/form/Form.svelte';
    import LabeledValue from '$lib5/LabeledValue.svelte';
    import type { GroupResponse, GroupRequest } from '$api/types/groups.ts';
    import { PATTERN_GROUP } from '$utils/patterns';
    import { untrack } from 'svelte';

    let {
        group,
        groups,
        onSave,
    }: {
        group: GroupResponse;
        groups: GroupResponse[];
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);
    let name = $state(untrack(() => group.name));

    $effect(() => {
        if (group.id) {
            name = group.name;
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (name !== group.name && groups.find(g => g.name === name)) {
            err = ta.common.nameExistsAlready;
            return;
        }

        let payload: GroupRequest = {
            group: name,
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

<Form action={`/auth/v1/groups/${group.id}`} {onSubmit}>
    <LabeledValue label="ID" mono>
        {group.id}
    </LabeledValue>

    <Input
        bind:value={name}
        autocomplete="off"
        label={ta.groups.name}
        placeholder={ta.groups.name}
        width="14.5rem"
        required
        pattern={PATTERN_GROUP}
    />

    <div class="flex gap-05">
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
</Form>

<style>
</style>
