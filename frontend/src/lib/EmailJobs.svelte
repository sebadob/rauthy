<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import IconEnvelope from '#icons/IconEnvelope.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import Modal from '#lib/Modal.svelte';
    import type { EmailJobResponse } from '#api/types/email.ts';
    import { fetchGet, fetchPost } from '#api/fetch.ts';
    import { formatDateFromTs } from '#utils/helpers.ts';
    import IconClock from '#icons/IconClock.svelte';
    import Expandable from '#lib/Expandable.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import IconStop from '#icons/IconStop.svelte';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import MarkdownRenderer from '#lib/text_edit/md/MarkdownRenderer.svelte';

    let ta = useI18nAdmin();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let jobs: EmailJobResponse[] = $state([]);

    $effect(() => {
        if (showModal) {
            fetchJobs();
        }
    });

    async function fetchJobs() {
        let res = await fetchGet<EmailJobResponse[]>('/auth/v1/email');
        if (res.body) {
            jobs = res.body;
        } else {
            console.error(res.error);
        }
    }

    async function cancelJob(job: EmailJobResponse) {
        if (job.status !== 'Open') {
            return;
        }

        let res = await fetchPost(`/auth/v1/email/cancel/${job.id}`);
        if (res.error) {
            console.error(res.error);
        } else {
            await fetchJobs();
        }
    }
</script>

<Button ariaLabel={ta.email.jobs} onclick={() => (showModal = true)} invisible>
    <span class="jobs-icon"><IconEnvelope /><span><IconClock width="0.875rem" /></span></span>
</Button>

<Modal
    bind:showModal
    bind:closeModal
    ariaLabel={ta.email.jobs}
    --dialog-padding="clamp(16px, 4vw, 32px)"
>
    <div class="jobs-panel">
        <h3>{ta.email.jobs}</h3>

        <div class="jobs">
            {#if jobs.length === 0}
                -
            {/if}

            {#each jobs as job (job.id)}
                <Expandable>
                    {#snippet summary()}
                        <div class="job-summary">
                            <div class="job-copy">
                                <strong
                                    >{job.subject.length > 79
                                        ? job.subject.slice(0, 79) + '…'
                                        : job.subject}</strong
                                >
                                <div class="job-meta">
                                    <time class="job-date">{formatDateFromTs(job.id)}</time>
                                    <span class="job-status">
                                        {#if job.status === 'Finished'}
                                            <IconCheck width="1rem" />
                                        {:else if job.status === 'Open'}
                                            <IconClock width="1rem" />
                                        {:else if job.status === 'Canceled'}
                                            <IconStop width="1rem" />
                                        {/if}
                                        {job.status}
                                    </span>
                                    {#if job.status !== 'Open'}
                                        <time class="job-date"
                                            >({formatDateFromTs(job.updated)})</time
                                        >
                                    {/if}
                                </div>
                            </div>
                            {#if job.status === 'Open'}
                                <Button
                                    ariaLabel={ta.email.cancelJob}
                                    invisible
                                    onclick={() => cancelJob(job)}
                                >
                                    <IconStop width="1rem" />
                                </Button>
                            {/if}
                        </div>
                    {/snippet}
                    {#snippet details()}
                        <div class="info">
                            <div>
                                <LabeledValue label={ta.email.scheduled}>
                                    {#if job.scheduled}
                                        {formatDateFromTs(job.scheduled)}
                                    {:else}
                                        {ta.email.immediate}
                                    {/if}
                                </LabeledValue>
                            </div>

                            <div>
                                <LabeledValue label="Update">
                                    {formatDateFromTs(job.updated)}
                                </LabeledValue>
                            </div>

                            <div>
                                <LabeledValue label="Filter">
                                    {job.filter_type}
                                    {job.filter_value}
                                </LabeledValue>
                            </div>
                        </div>
                        <LabeledValue label={ta.email.subject}>
                            {job.subject}
                        </LabeledValue>

                        <div class="emailBody">
                            {#if job.content_type === 'text'}
                                {job.body}
                            {:else if job.content_type === 'markdown'}
                                <MarkdownRenderer markdown={job.body} />
                            {:else}
                                {@html job.body}
                            {/if}
                        </div>
                    {/snippet}
                </Expandable>
            {/each}
        </div>
    </div>
</Modal>

<style>
    .jobs-icon {
        display: grid;
        position: relative;
    }

    .jobs-icon > span {
        position: absolute;
        inset: auto -0.2rem -0.125rem auto;
        border-radius: 50%;
        background: var(--ui-surface);
    }

    .emailBody {
        margin: 0.5rem 0;
        padding: 0.25rem 0.5rem;
        background: hsla(var(--bg-high) / 0.1);
        border-radius: var(--border-radius);
        overflow-wrap: anywhere;
        overflow-x: auto;
    }

    .info {
        display: flex;
        justify-content: space-between;
        flex-wrap: wrap;
    }

    .info > div {
        flex: 1 1 10.5rem;
        min-width: 0;
    }

    .jobs-panel {
        width: min(40rem, calc(100dvw - 34px - 2 * var(--dialog-padding, 32px)));
        min-height: 15rem;
        overflow-wrap: anywhere;
    }

    .jobs {
        display: grid;
        gap: 0.5rem;
    }

    .job-summary {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        --ui-button-padding: 0 0.375rem;
    }

    .job-copy {
        flex: 1;
        min-width: 0;
    }

    .job-copy strong {
        color: var(--ui-text);
    }

    .job-meta {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0.25rem 0.75rem;
        margin-top: 0.25rem;
        font-size: 0.8125rem;
        line-height: 1.25rem;
    }

    .job-date {
        color: var(--ui-muted);
    }

    .job-status {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
    }

    @media (width < 520px) {
        .jobs :global(.admin-expandable > .header) {
            gap: 0.375rem;
            padding: 0.75rem 0.5rem;
        }

        .job-meta > :first-child {
            flex-basis: 100%;
        }
    }
</style>
