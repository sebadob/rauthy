<script lang="ts">
    import { onMount } from 'svelte';
    import Section from '#lib/Section.svelte';
    import Argon2Utility from '#lib/admin/config/argon2/Argon2Utility.svelte';
    import { fetchGet } from '#api/fetch.ts';
    import type { Argon2ParamsResponse, LoginTimeResponse } from '#api/types/login_times.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';

    let ta = useI18nAdmin();
    let t = useI18n();

    let params: undefined | Argon2ParamsResponse = $state();
    let loginTime: undefined | number = $state();
    let numCpus: undefined | number = $state();

    onMount(async () => {
        let res = await fetchGet<LoginTimeResponse>('/auth/v1/login_time');
        if (res.body) {
            params = res.body.argon2_params;
            loginTime = res.body.login_time;
            numCpus = res.body.num_cpus;
        } else {
            console.error(res.error);
        }
    });
</script>

<h2 class="ui-page-title">Argon2ID - Password Hashing</h2>

<div class="ui-section-stack">
    <Section title={ta.docs.hashing.currValuesHead}>
        <p class="ui-muted-copy">{ta.docs.hashing.currValues1}</p>
        {#if params}
            <div class="stats ui-field-grid">
                <div><span>m_cost</span><strong>{params.m_cost} kB</strong></div>
                <div><span>p_cost</span><strong>{params.p_cost} threads</strong></div>
                <div><span>t_cost</span><strong>{params.t_cost}</strong></div>
                <div><span>Login Time: </span><strong>{loginTime} ms</strong></div>
                <div>
                    <span>{ta.docs.hashing.currValuesThreadsAccess}: </span><strong
                        >{numCpus}</strong
                    >
                </div>
            </div>
        {/if}
        <p class="ui-muted-copy">{ta.docs.hashing.currValuesNote}</p>
    </Section>

    <Section title={ta.docs.hashing.utilityHead}>
        <p class="ui-muted-copy">{ta.docs.hashing.pUtility}</p>
        {#if params}<Argon2Utility {params} />{/if}
        <details>
            <summary>{t.common.details}</summary>
            <p class="ui-muted-copy">{ta.docs.hashing.utility1}</p>
            <p class="ui-muted-copy">{@html ta.docs.hashing.utility2}</p>
        </details>
    </Section>

    <Section>
        <details>
            <summary>{ta.docs.hashing.tune}</summary>
            <p class="ui-muted-copy">{ta.docs.hashing.pTune}</p>
            <p class="ui-muted-copy">{ta.docs.hashing.pDetails}</p>
            <div class="params ui-field-grid">
                <div>
                    <b>m_cost</b>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.mCost1}</p>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.mCost2}</p>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.mCost3}</p>
                </div>

                <div>
                    <b>p_cost</b>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.pCost1}</p>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.pCost2}</p>
                </div>

                <div>
                    <b>t_cost</b>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.tCost1}</p>
                    <p class="ui-muted-copy">{@html ta.docs.hashing.tCost2}</p>
                </div>
            </div>
        </details>
        <details>
            <summary>{ta.docs.hashing.loginTimeHead}</summary>
            <p class="ui-muted-copy">{ta.docs.hashing.loginTime1}</p>
            <p class="ui-muted-copy">{ta.docs.hashing.loginTime2}</p>
        </details>
    </Section>
</div>

<style>
    .stats {
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 9rem), 1fr));
    }

    .stats > div {
        display: grid;
        align-content: start;
        gap: 0.25rem;
        padding: 0.625rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-container);
    }

    .stats strong {
        font-variant-numeric: tabular-nums;
    }

    p {
        margin: 0;
    }

    .stats span {
        color: var(--ui-muted);
        font-size: 0.8125rem;
    }

    details {
        padding: 0.625rem 0.75rem;
        border-radius: var(--border-radius);
        background: var(--ui-container);
    }

    summary {
        cursor: pointer;
        font-size: var(--ui-control-font-size);
    }

    .params > div {
        min-width: 0;
        padding: 0.75rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
    }

    details p,
    .params {
        margin-top: 0.5rem;
    }
</style>
