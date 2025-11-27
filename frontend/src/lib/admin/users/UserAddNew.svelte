<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import { PATTERN_USER_NAME } from '$utils/patterns';
    import Form from '$lib5/form/Form.svelte';
    import { fetchPost } from '$api/fetch';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import type { Language } from '$api/types/i18n.ts';
    import type { NewUserRequest, UserResponse } from '$api/types/user.ts';
    import Options from '$lib5/Options.svelte';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import InputDateTimeCombo from '$lib5/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '$utils/form';
    import { slide } from 'svelte/transition';
    import type { RoleResponse } from '$api/types/roles.ts';
    import type { GroupResponse } from '$api/types/groups.ts';
    import SelectList from '$lib5/select_list/SelectList.svelte';
    import type { SelectItem } from '$lib5/select_list/props.ts';
    import { untrack } from 'svelte';
    import LabeledValue from '$lib5/LabeledValue.svelte';
    import { useI18nConfig } from '$state/i18n_config.svelte';
    import type { UserValuesConfig } from '$api/templates/UserValuesConfig';

    let {
        config,
        onSave,
        roles,
        groups,
    }: {
        config: UserValuesConfig | undefined;
        onSave: (id: string) => void;
        roles: RoleResponse[];
        groups: GroupResponse[];
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let ref: undefined | HTMLInputElement = $state();

    let err = $state('');
    let email = $state('');
    let givenName = $state('');
    let familyName = $state('');
    let languages = $derived(
        useI18nConfig()
            .common()
            ?.map(l => l as string),
    );
    let language: Language = $state('en');

    let rolesItems: SelectItem[] = $state(
        untrack(() =>
            roles.map(r => {
                let i: SelectItem = {
                    name: r.name,
                    selected: false,
                };
                return i;
            }),
        ),
    );
    let groupsItems: SelectItem[] = $state(
        untrack(() =>
            groups.map(g => {
                let i: SelectItem = {
                    name: g.name,
                    selected: false,
                };
                return i;
            }),
        ),
    );

    let expires = $state(false);
    let expDate = $state(fmtDateInput());
    let expTime = $state(fmtTimeInput());

    $effect(() => {
        requestAnimationFrame(() => {
            ref?.focus();
        });
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        let payload: NewUserRequest = {
            email,
            given_name: givenName || undefined,
            family_name: familyName || undefined,
            language,
            groups: groupsItems.filter(i => i.selected).map(i => i.name),
            roles: rolesItems.filter(i => i.selected).map(i => i.name),
            user_expires: expires ? unixTsFromLocalDateTime(expDate, expTime) : undefined,
        };
        if (payload.groups?.length === 0) {
            payload.groups = undefined;
        }
        let res = await fetchPost<UserResponse>(form.action, payload);
        if (res.body) {
            onSave(res.body.id);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<div class="container">
    <Form action="/auth/v1/users" {onSubmit}>
        <Input
            typ="email"
            bind:ref
            bind:value={email}
            autocomplete="off"
            label="E-Mail"
            placeholder="E-Mail"
            required
        />
        <Input
            bind:value={givenName}
            autocomplete="off"
            label={t.account.givenName}
            placeholder={t.account.givenName}
            required={config?.given_name === 'required'}
            pattern={PATTERN_USER_NAME}
        />
        <Input
            bind:value={familyName}
            autocomplete="off"
            label={t.account.familyName}
            placeholder={t.account.familyName}
            required={config?.family_name === 'required'}
            pattern={PATTERN_USER_NAME}
        />

        {#if languages}
            <LabeledValue label={ta.common.language}>
                <Options
                    ariaLabel={t.common.selectI18n}
                    options={languages}
                    bind:value={language}
                    borderless
                />
            </LabeledValue>
        {/if}

        <SelectList bind:items={rolesItems}>
            {t.account.roles}
        </SelectList>
        <SelectList bind:items={groupsItems}>
            {t.account.groups}
        </SelectList>

        <InputCheckbox ariaLabel={t.account.accessExp} bind:checked={expires}>
            {t.account.accessExp}
        </InputCheckbox>
        {#if expires}
            <div transition:slide={{ duration: 150 }}>
                <InputDateTimeCombo
                    label={t.account.accessExp}
                    bind:value={expDate}
                    bind:timeValue={expTime}
                    withTime
                    min={fmtDateInput()}
                    required
                />
            </div>
        {/if}

        <div style:height=".66rem"></div>
        <Button type="submit">
            {t.common.save}
        </Button>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </Form>
</div>

<style>
    .container {
        width: min(calc(100dvw - 1.75rem), 22rem);
        min-height: 33.5rem;
        text-align: left;
    }
</style>
