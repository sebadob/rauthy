<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {fetchPut} from "$api/fetch.ts";
    import Form from "$lib5/form/Form.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import type {RoleRequest, RoleResponse} from "$api/types/roles.ts";
    import {PATTERN_GROUP} from "$utils/patterns.ts";

    let {
        role,
        roles,
        onSave,
    }: {
        role: RoleResponse,
        roles: RoleResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);
    let isRauthyAdmin = $derived(role.name === 'rauthy_admin');

    let name = $state(role.name);

    $effect(() => {
        if (role.id) {
            name = role.name;
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (name !== role.name && roles.find(r => r.name === name)) {
            err = ta.common.nameExistsAlready;
            return;
        }

        let payload: RoleRequest = {
            role: name,
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

<Form action={`/auth/v1/roles/${role.id}`} {onSubmit}>
    <LabeledValue label="ID" mono>
        {role.id}
    </LabeledValue>

    <Input
            bind:value={name}
            autocomplete="off"
            label={ta.scopes.name}
            placeholder={ta.scopes.name}
            disabled={isRauthyAdmin}
            width="14.5rem"
            required
            pattern={PATTERN_GROUP}
    />

    {#if isRauthyAdmin}
        <p>{@html ta.roles.adminNoMod}</p>
    {:else}
        <div class="flex gap-05">
            <Button type="submit">
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck/>
            {/if}
        </div>
    {/if}

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</Form>

<style>
</style>
