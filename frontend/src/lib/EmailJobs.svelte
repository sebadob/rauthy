<script lang="ts">
    import Button from '$lib/button/Button.svelte';
    import IconCircleStack from '$icons/IconCircleStack.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Modal from '$lib/Modal.svelte';
    import type { EmailJobResponse } from '$api/types/email';
    import { fetchGet, fetchPost } from '$api/fetch';
    import { formatDateFromTs } from '$utils/helpers';
    import IconClock from '$icons/IconClock.svelte';
    import Expandable from '$lib/Expandable.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import IconStop from '$icons/IconStop.svelte';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import MarkdownRenderer from '$lib/text_edit/md/MarkdownRenderer.svelte';

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

<Button onclick={() => (showModal = true)} invisible>
    <IconCircleStack />
</Button>

<Modal bind:showModal bind:closeModal>
    <div class="modal">
        <h3>{ta.email.jobs}</h3>

        <div class="jobs">
            {#if jobs.length === 0}
                -
            {/if}

            {#each jobs as job}
                <Expandable>
                    {#snippet summary()}
                        <div>
                            <div class="flex gap-10">
                                <div>{formatDateFromTs(job.id)}</div>
                                <div class="flex gap-05">
                                    {#if job.status === 'Finished'}
                                        <IconCheck />
                                    {:else if job.status === 'Open'}
                                        <IconClock />
                                    {:else if job.status === 'Canceled'}
                                        <IconStop />
                                    {/if}
                                    {job.status}
                                    {#if job.status !== 'Open'}
                                        <span class="muted">({formatDateFromTs(job.updated)})</span>
                                    {/if}
                                </div>
                                {#if job.status === 'Open'}
                                    <Button
                                        ariaLabel={ta.email.cancelJob}
                                        invisible
                                        onclick={() => cancelJob(job)}
                                    >
                                        <IconStop />
                                    </Button>
                                {/if}
                            </div>
                            <div>
                                <b>
                                    {#if job.subject.length > 79}
                                        {job.subject.slice(0, 79)}(...)
                                    {:else}
                                        {job.subject}
                                    {/if}
                                </b>
                            </div>
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
    .emailBody {
        margin: 0.5rem 0;
        padding: 0.25rem 0.5rem;
        background: hsla(var(--bg-high) / 0.1);
        border-radius: var(--border-radius);
    }

    .info {
        display: flex;
        justify-content: space-between;
        flex-wrap: wrap;
    }

    .info > div {
        min-width: 10.5rem;
        flex: 1;
    }

    .modal {
        max-height: 90dvh;
        min-height: 15rem;
        width: min(calc(467pt + 3rem), 90dvw);
    }

    .muted {
        color: hsla(var(--text) / 0.6);
    }
</style>
