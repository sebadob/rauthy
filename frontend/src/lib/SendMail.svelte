<script lang="ts">
    import IconEnvelope from '$icons/IconEnvelope.svelte';
    import Button from '$lib/button/Button.svelte';
    import Modal from '$lib/Modal.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Options from '$lib/Options.svelte';
    import EditorInteractive from '$lib/text_edit/EditorInteractive.svelte';
    import type { GroupResponse } from '$api/types/groups';
    import type { RoleResponse } from '$api/types/roles';
    import Input from '$lib/form/Input.svelte';
    import type { EmailJobRequest } from '$api/types/email';
    import InputDateTimeCombo from '$lib/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '$utils/form';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { fetchPost } from '$api/fetch';
    import IconCheck from '$icons/IconCheck.svelte';

    let {
        groups,
        roles,
    }: {
        groups: GroupResponse[];
        roles: RoleResponse[];
    } = $props();

    let ta = useI18nAdmin();

    let showModal = $state(true);
    let closeModal: undefined | (() => void) = $state();

    let editorMode: 'Text' | 'Markdown' | 'HTML' = $state('Markdown');

    let filter = $state(ta.email.filterType[0]);
    let group = $state('');
    let role = $state('');

    let scheduled = $state(false);
    let schedDate = $state(fmtDateInput());
    let schedTime = $state(fmtTimeInput());

    let subject = $state('');
    let body = $state('');
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
        if (groups.length > 0) {
            group = groups[0].name;
        }
    });

    $effect(() => {
        if (roles.length > 0) {
            role = roles[0].name;
        }
    });

    async function submit() {
        let payload: EmailJobRequest = {
            // scheduled: number;
            filter_type: 'none',
            content_type: editorMode === 'Text' ? 'text' : 'markdown',
            subject,
            body: editorMode === 'HTML' ? body : bodyRaw,
        };

        if (scheduled) {
            payload.scheduled = unixTsFromLocalDateTime(schedDate, schedTime);
        }
        if (isFilterGroup) {
            payload.filter_value = group;
            if (filter === ta.email.filterType[1]) {
                payload.filter_type = 'in_group';
            } else {
                payload.filter_type = 'not_in_group';
            }
        }
        if (isFilterRole) {
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
        filter = ta.email.filterType[0];
        subject = '';
        bodyRaw = '';
        body = '';
        success = false;
        error = '';
    }
</script>

<Button ariaLabel={ta.email.sendMail} onclick={() => (showModal = true)} invisible>
    <IconEnvelope />
</Button>

<Modal bind:showModal bind:closeModal>
    <div class="options">
        <div class="scheduled">
            <InputCheckbox ariaLabel={ta.email.scheduled} bind:checked={scheduled}>
                {ta.email.scheduled}
            </InputCheckbox>

            {#if scheduled}
                <InputDateTimeCombo
                    bind:value={schedDate}
                    bind:timeValue={schedTime}
                    min={fmtDateInput()}
                    withTime
                />
            {/if}
        </div>

        <div class="filter">
            <div class="label">
                {ta.email.userFilter}
                <span style:margin=".25rem 0">:</span>
            </div>

            <Options
                ariaLabel={ta.email.userFilter}
                options={ta.email.filterType}
                bind:value={filter}
                borderless
            />

            {#if filter !== ta.email.filterType[0]}
                <div class="ms-05">:</div>
                {#if isFilterGroup}
                    <Options
                        ariaLabel={ta.email.userFilter}
                        options={groups.map(g => g.name)}
                        bind:value={group}
                        borderless
                        withSearch={groups.length > 7}
                    />
                {:else if isFilterRole}
                    <Options
                        ariaLabel={ta.email.userFilter}
                        options={roles.map(g => g.name)}
                        bind:value={role}
                        borderless
                        withSearch={roles.length > 7}
                    />
                {/if}
            {/if}
        </div>
    </div>

    <div class="editor">
        <Input
            label={ta.email.subject}
            placeholder={ta.email.subject}
            bind:value={subject}
            width="min(40rem, 90dvw)"
            maxLength={1024}
            required
        />
        <EditorInteractive
            bind:mode={editorMode}
            bind:contentRaw={bodyRaw}
            bind:sanitizedValue={body}
            height="min(60dvh, 40rem)"
        />
    </div>

    <div class="send">
        <p>
            {#if isFilterNone}
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
            <Button onclick={submit} isDisabled={!body || !subject}>Send</Button>

            {#if success}
                <IconCheck />
            {:else if error}
                <span class="err">
                    {error}
                </span>
            {/if}
        </div>
    </div>
</Modal>

<style>
    .editor {
        min-width: min(40rem, 93dvw);
    }

    .err {
        margin-top: 0.25rem;
    }

    .filter {
        margin-top: -0.1rem;
        margin-bottom: 0.5rem;
        display: flex;
        align-items: center;
        font-size: 0.98rem;
        flex-wrap: wrap;
    }

    .options {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .scheduled {
        min-width: 15rem;
    }

    .send {
        margin-top: 1rem;
    }

    @media (min-width: 1440px) {
        .editor {
            min-width: 80rem;
        }
    }
</style>
