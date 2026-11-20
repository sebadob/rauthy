<script lang="ts">
    import IconEnvelope from '#icons/IconEnvelope.svelte';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import Options from '#lib/Options.svelte';
    import EditorInteractive from '#lib/text_edit/EditorInteractive.svelte';
    import type { GroupResponse } from '#api/types/groups.ts';
    import type { RoleResponse } from '#api/types/roles.ts';
    import Input from '#lib/form/Input.svelte';
    import type { EmailJobRequest } from '#api/types/email.ts';
    import InputDateTimeCombo from '#lib/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '#utils/form.ts';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import { fetchPost } from '#api/fetch.ts';
    import IconCheck from '#icons/IconCheck.svelte';
    import { useSession } from '#state/session.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        groups,
        roles,
    }: {
        groups: GroupResponse[];
        roles: RoleResponse[];
    } = $props();

    let ta = useI18nAdmin();
    let t = useI18n();
    let session = useSession('admin');

    // A delegated group admin may only email users of a single group it manages, so
    // the filter is locked to "In Group" and the group dropdown is limited to managed groups.
    // The backend enforces the same. The principal acts as a group admin when it reached the
    // admin UI without the full `rauthy_admin` role.
    let isGroupAdmin = $derived(!session.isAdmin());
    let availableGroups = $derived(
        isGroupAdmin ? groups.filter(g => session.managesGroup(g.name)) : groups,
    );

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let editorMode: 'Text' | 'Markdown' = $state('Markdown');

    let filter = $state(ta.email.filterType[0]);
    let group = $state('');
    let role = $state('');

    let scheduled = $state(false);
    const defaultDispatch = new Date(Date.now() + 5 * 60_000);
    let schedDate = $state(fmtDateInput(defaultDispatch));
    let schedTime = $state(fmtTimeInput(defaultDispatch));
    let scheduledAt = $derived(unixTsFromLocalDateTime(schedDate, schedTime));

    let subject = $state('');
    let bodyRaw = $state('');

    let success = $state(false);
    let error = $state('');

    let isFilterNone = $derived(filter === ta.email.filterType[0]);
    let isFilterGroup = $derived(
        filter === ta.email.filterType[1] || filter === ta.email.filterType[2],
    );
    let isFilterRole = $derived(
        filter === ta.email.filterType[3] || filter === ta.email.filterType[4],
    );

    $effect(() => {
        // default to the first selectable group, and keep `group` valid if the list narrows
        // for a group admin
        if (availableGroups.length > 0 && !availableGroups.some(g => g.name === group)) {
            group = availableGroups[0].name;
        }
    });

    $effect(() => {
        if (roles.length > 0 && !roles.some(r => r.name === role)) {
            role = roles[0].name;
        }
    });

    function openComposer() {
        if (!scheduled) {
            const dispatch = new Date(Date.now() + 5 * 60_000);
            schedDate = fmtDateInput(dispatch);
            schedTime = fmtTimeInput(dispatch);
        }
        showModal = true;
    }

    async function submit() {
        if (scheduled && (!scheduledAt || scheduledAt <= Date.now() / 1000)) {
            error = t.common.invalidInput;
            return;
        }
        error = '';
        let payload: EmailJobRequest = {
            filter_type: 'none',
            content_type: editorMode === 'Text' ? 'text' : 'markdown',
            subject,
            body: bodyRaw,
        };

        if (scheduled) {
            payload.scheduled = scheduledAt;
        }
        if (isGroupAdmin) {
            // group admins are locked to an "In Group" filter for a group they manage
            payload.filter_type = 'in_group';
            payload.filter_value = group;
        } else if (isFilterGroup) {
            payload.filter_value = group;
            if (filter === ta.email.filterType[1]) {
                payload.filter_type = 'in_group';
            } else {
                payload.filter_type = 'not_in_group';
            }
        }
        if (!isGroupAdmin && isFilterRole) {
            payload.filter_value = role;
            if (filter === ta.email.filterType[3]) {
                payload.filter_type = 'has_role';
            } else {
                payload.filter_type = 'has_not_role';
            }
        }

        let res = await fetchPost('/auth/v1/email', payload);
        if (res.status === 200) {
            success = true;
            setTimeout(() => {
                closeModal?.();
                reset();
            }, 3000);
        } else {
            error = res.error?.message || 'Error';
        }
    }

    function reset() {
        scheduled = false;
        editorMode = 'Markdown';
        filter = ta.email.filterType[0];
        subject = '';
        bodyRaw = '';
        success = false;
        error = '';
    }
</script>

<Button ariaLabel={ta.email.sendMail} onclick={openComposer} invisible>
    <IconEnvelope />
</Button>

<Modal bind:showModal bind:closeModal ariaLabel={ta.email.sendMail}>
    <div class="admin-mail-composer">
        <h2>{ta.email.sendMail}</h2>
        <div class="options">
            <div class="filter">
                <span class="admin-field-label">{ta.email.userFilter}</span>
                <div class="filter-options">
                    {#if isGroupAdmin}
                        <span>{ta.email.filterType[1]}</span>
                    {:else}
                        <Options
                            ariaLabel={ta.email.userFilter}
                            options={ta.email.filterType}
                            bind:value={filter}
                        />
                    {/if}

                    {#if isGroupAdmin || isFilterGroup}
                        <Options
                            ariaLabel={ta.nav.groups}
                            options={availableGroups.map(g => g.name)}
                            bind:value={group}
                            withSearch={availableGroups.length > 7}
                        />
                    {:else if isFilterRole}
                        <Options
                            ariaLabel={ta.nav.roles}
                            options={roles.map(r => r.name)}
                            bind:value={role}
                            withSearch={roles.length > 7}
                        />
                    {/if}
                </div>
            </div>
            <div class="scheduled">
                <InputCheckbox ariaLabel={ta.email.scheduled} bind:checked={scheduled}>
                    {ta.email.scheduled}
                </InputCheckbox>
                {#if scheduled}
                    <div class="schedule-fields">
                        <InputDateTimeCombo
                            label={ta.common.date}
                            labelTime={ta.common.time}
                            bind:value={schedDate}
                            bind:timeValue={schedTime}
                            min={fmtDateInput()}
                            required
                            withTime
                        />
                    </div>
                {/if}
            </div>
        </div>

        <div class="editor">
            <Input
                label={ta.email.subject}
                placeholder={ta.email.subject}
                bind:value={subject}
                width="100%"
                maxLength={1024}
                required
            />
            <EditorInteractive
                bind:mode={editorMode}
                bind:contentRaw={bodyRaw}
                height="clamp(11rem, 28dvh, 21rem)"
            />
        </div>

        <div class="send">
            <p>
                {#if isGroupAdmin}
                    {ta.email.sendAllUsersFiltered}
                    <b>{`${ta.email.filterType[1]} ${group}`}</b>
                {:else if isFilterNone}
                    {ta.email.sendAllUsers}
                {:else if isFilterGroup}
                    {ta.email.sendAllUsersFiltered}
                    <b>{`${filter} ${group}`}</b>
                {:else if isFilterRole}
                    {ta.email.sendAllUsersFiltered}
                    <b>{`${filter} ${role}`}</b>
                {/if}
            </p>

            <div class="flex gap-10">
                <Button
                    onclick={submit}
                    isDisabled={!bodyRaw.trim() ||
                        !subject.trim() ||
                        ((isGroupAdmin || isFilterGroup) && !group) ||
                        (isFilterRole && !role)}>Send</Button
                >

                {#if success}
                    <IconCheck />
                {:else if error}
                    <span class="err">
                        {error}
                    </span>
                {/if}
            </div>
        </div>
    </div>
</Modal>

<style>
    .admin-mail-composer {
        width: min(720px, calc(100dvw - 96px));
        max-width: 100%;
    }
    h2 {
        margin: 0 0 1.25rem;
        font-size: 1.375rem;
        line-height: 1.75rem;
        color: var(--ui-text);
    }
    .options {
        display: grid;
        gap: 1rem;
        margin-bottom: 1.25rem;
    }
    .filter,
    .editor {
        min-width: 0;
    }
    .filter-options {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0.5rem;
    }
    .schedule-fields {
        width: min(100%, 25rem);
        margin-top: 0.625rem;
    }
    .send {
        margin-top: 1rem;
    }
    .send p {
        max-width: none;
        font-size: 0.8125rem;
    }
</style>
