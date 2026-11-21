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

    const deliveryId = $props.id();
    let showModal = $state(false);
    let expanded = $state(false);
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

<Modal
    bind:showModal
    bind:closeModal
    ariaLabel={ta.email.sendMail}
    {expanded}
    onClose={() => (expanded = false)}
    --dialog-padding="clamp(14px, 3vw, 24px)"
>
    <EditorInteractive
        bind:mode={editorMode}
        bind:expanded
        bind:contentRaw={bodyRaw}
        title={ta.email.sendMail}
    >
        {#snippet fields()}
            <div class="subject">
                <Input
                    label={ta.email.subject}
                    placeholder={ta.email.subject}
                    bind:value={subject}
                    width="100%"
                    maxLength={1024}
                    required
                />
            </div>
            <div class="delivery-fields">
                {#if !isGroupAdmin}
                    <div class="delivery-field admin-field-select">
                        <span class="admin-field-label">{ta.email.userFilter}</span>
                        <Options
                            ariaLabel={ta.email.userFilter}
                            options={ta.email.filterType}
                            bind:value={filter}
                        />
                    </div>
                {/if}
                {#if isGroupAdmin || isFilterGroup}
                    <div class="delivery-field admin-field-select">
                        <span class="admin-field-label">{ta.nav.groups}</span>
                        <Options
                            ariaLabel={ta.nav.groups}
                            options={availableGroups.map(g => g.name)}
                            bind:value={group}
                            withSearch={availableGroups.length > 7}
                        />
                    </div>
                {:else if isFilterRole}
                    <div class="delivery-field admin-field-select">
                        <span class="admin-field-label">{ta.nav.roles}</span>
                        <Options
                            ariaLabel={ta.nav.roles}
                            options={roles.map(r => r.name)}
                            bind:value={role}
                            withSearch={roles.length > 7}
                        />
                    </div>
                {/if}
                <div class="delivery-field">
                    <label class="admin-field-label" for={deliveryId}
                        >{ta.email.delivery ?? 'Delivery'}</label
                    >
                    <select
                        id={deliveryId}
                        class="ui-text-input"
                        bind:value={
                            () => (scheduled ? 'scheduled' : 'immediate'),
                            value => (scheduled = value === 'scheduled')
                        }
                    >
                        <option value="immediate">{ta.email.immediate}</option>
                        <option value="scheduled">
                            {ta.email.scheduledDelivery ?? ta.email.scheduled}
                        </option>
                    </select>
                </div>
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
        {/snippet}
        {#snippet footer()}
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
        {/snippet}
    </EditorInteractive>
</Modal>

<style>
    .delivery-fields {
        display: flex;
        flex-wrap: wrap;
        align-items: flex-start;
        gap: 0.5rem 0.75rem;
    }

    .delivery-field {
        flex: 1 1 8.5rem;
        min-width: 0;
    }

    .delivery-field select {
        height: var(--ui-field-height);
        padding-block: 0;
        text-overflow: ellipsis;
    }

    .schedule-fields {
        flex: 2 1 18rem;
        min-width: 0;
    }

    @media (min-width: 360px) {
        .schedule-fields {
            --ui-datetime-compact-columns: minmax(0, 1fr) minmax(5.5rem, 0.65fr);
        }
    }

    .subject {
        --ui-field-margin: 0;
        margin-bottom: 0.5rem;
    }

    .send p {
        margin: 0 0 0.5rem;
        max-width: none;
        font-size: 0.8125rem;
    }
</style>
