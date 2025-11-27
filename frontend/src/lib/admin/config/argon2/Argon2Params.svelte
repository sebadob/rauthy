<script lang="ts">
    import { onMount } from 'svelte';
    import Argon2Utility from '$lib5/admin/config/argon2/Argon2Utility.svelte';
    import { fetchGet } from '$api/fetch';
    import type { Argon2ParamsResponse, LoginTimeResponse } from '$api/types/login_times.ts';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';

    let ta = useI18nAdmin();

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

<h3>Argon2ID - Password Hashing</h3>
<p>{ta.docs.hashing.pUtility}</p>
<p>
    <b>{ta.docs.hashing.tune}</b><br />
    {ta.docs.hashing.pTune}
</p>
<p>{ta.docs.hashing.pDetials}</p>
<ul>
    <li>
        <b>m_cost</b>
        <p>{@html ta.docs.hashing.mCost1}</p>
        <p>{@html ta.docs.hashing.mCost2}</p>
        <p>{@html ta.docs.hashing.mCost3}</p>
    </li>

    <li>
        <b>p_cost</b>
        <p>{@html ta.docs.hashing.pCost1}</p>
        <p>{@html ta.docs.hashing.pCost2}</p>
    </li>

    <li>
        <b>t_cost</b>
        <p>{@html ta.docs.hashing.tCost1}</p>
        <p>{@html ta.docs.hashing.tCost2}</p>
    </li>
</ul>

<br />
<h3>{ta.docs.hashing.loginTimeHead}</h3>
<p>{ta.docs.hashing.loginTime1}</p>
<p>{ta.docs.hashing.loginTime2}</p>
<br />

<h3>{ta.docs.hashing.currValuesHead}</h3>
<p>{ta.docs.hashing.currValues1}</p>
<div>
    <div class="flex">
        <div class="label">m_cost:</div>
        {params?.m_cost} kB
    </div>

    <div class="flex">
        <div class="label">p_cost:</div>
        {params?.p_cost} threads
    </div>

    <div class="flex">
        <div class="label">t_cost:</div>
        {params?.t_cost}
    </div>

    <br />
    <div class="flex">
        <div class="label">Login Time:</div>
        {loginTime} ms
    </div>

    <p>{ta.docs.hashing.currValuesNote}</p>
    <b>{ta.docs.hashing.currValuesThreadsAccess}</b>: {numCpus}<br />
</div>
<br />

<h3>{ta.docs.hashing.utilityHead}</h3>
<p>{ta.docs.hashing.utility1}</p>
<p>{@html ta.docs.hashing.utility2}</p>

{#if params}
    <Argon2Utility {params} />
{/if}

<div style="height: 1.5rem"></div>

<style>
    .label {
        width: 7rem;
        font-weight: bold;
    }
</style>
