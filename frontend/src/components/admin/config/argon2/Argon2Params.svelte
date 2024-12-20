<script>
    import {onMount} from "svelte";
    import {getLoginTime} from "../../../../utils/dataFetchingAdmin.js";
    import Argon2Utility from "./Argon2Utility.svelte";

    let err = '';
    let currParams = $state();
    let loginTime = $state();
    let numCpus = $state();
    let isLoading = true;

    onMount(() => {
        fetchData();
    });

    async function fetchData() {
        let res = await getLoginTime();
        let body = await res.json();
        if (res.ok) {
            currParams = body.argon2_params;
            loginTime = body.login_time;
            numCpus = body.num_cpus;
        } else {
            err = body.message;
        }
        isLoading = false;
    }

    function onSave() {
        fetchData();
    }
</script>

<div class="container">
    <h3>Argon2ID - Password Hashing</h3>
    <p>
        This utility helps you find the best argon2id settings for your platform.<br>
        Argon2id is currently the safest available password hashing algorithm. To use it to its fullest potential, it
        has
        to be tuned for each deployment.
    </p>
    <p>
        <b>Important:These values need to be tuned on the final architecture!</b><br>
        They change depending on the capabilities of the system. The more powerful the system, the more safe these
        values
        can be.
    </p>
    <p>
        If you want a detailed introduction to argon2id, many sources exist online. This guide will just give very short
        overview about the values.<br>
        Three of them need to be configured:
    </p>
    <ul style:margin-left="-10px">
        <li>
            <b>m cost</b>
            <p>
                The <code>m_cost</code> defines the amount of <b>memory (in kB)</b>, which is used for the hashing.<br>
                The higher the value, the better, of course. But you need to keep in mind the servers resources.<br>
                When you hash 4 passwords at the same time, for instance, the backend needs <code>4 x m_cost</code>
                during the
                hashing. These resources must be available.
            </p>
            <p>
                Tuning <code>m_cost</code> is pretty easy. If you can give rauthy 1GB of memory for hashing operations,
                give it
                1GB. If you can sacrifice 4GB, give it 4GB.
            </p>
            <p>
                <b>Important: You should never go below an <code>m_cost</code> of <code>32768</code></b>.
            </p>
        </li>

        <li>
            <b>p cost</b>
            <p>
                The <code>p_cost</code> defines the amount of <b>parallelism</b> for hashing.<br>
                This value most often tops out at ~8, which is the default for Rauthy.<br>
                <code>p_cost</code> does not affect the time needed for the calculation, if the resources are available.
            </p>
            <p>
                The general rule is:<br>
                Set the <code>p_cost</code> to twice the size of cores your have available.<br>
                For instance, if you have 4 cores available, set the <code>p_cost</code> to <code>8</code>.<br>
            </p>
        </li>

        <li>
            <b>t cost</b>
            <p>
                The <code>t_cost</code> defines the amount of <b>time</b> for hashing.<br>
                This value is actually the only one, that needs tuning, since <code>m_cost</code> and
                <code>p_cost</code> are
                basically given by the environment.
            </p>
            <p>
                Tuning is easy: Set <code>m_cost</code> and <code>p_cost</code> accordingly and then increase <code>t_cost</code>
                as long as you have not reached your hashing-time-goal.
            </p>
        </li>
    </ul>

    <br>
    <h3>A word about Login Time</h3>
    <p>
        Generally, users want everything as fast as possible. When doing a safe login though, a time between 500 - 1000
        ms
        should not be a problem.<br>
        The login time must not be too short, since it would lower the strength of the hash, of course.
    </p>
    <p>
        To provide as much safety by default as possible, this utility does not allow you to go below 500 ms for the
        login
        time.
    </p>
    <br>

    <h3>Current values</h3>
    <p>
        The current values from the backend are the following:
    </p>
    <div>
        <div class="valRow">
            <div class="label">
                <b>m_cost</b>:
            </div>
            {currParams?.m_cost}
        </div>

        <div class="valRow">
            <div class="label">
                <b>p_cost</b>:
            </div>
            {currParams?.p_cost}
        </div>

        <div class="valRow">
            <div class="label">
                <b>t_cost</b>:
            </div>
            {currParams?.t_cost}
        </div>

        <br>
        <div class="valRow">
            <div class="label">
                <b>Login Time</b>:
            </div>
            {loginTime} ms
        </div>

        <p>
            <i>Note:</i> The Login Time from the backend does only provide a good guideline after at least 5 successful
            logins, after
            rauthy has been started.<br>
            The base value is always 2000 ms after a fresh restart and will adjust over time with each successful login.
        </p>

        <b>Threads (p_cost) rauthy has access to</b>: {numCpus}<br>
    </div>
    <br>

    <h3>Parameter Calculation Utility</h3>
    <p>
        You can use this tool to approximate good values for your deployment.<br>
        Keep in mind, that this should be executed with rauthy in its final place with all final resources
        available.<br><br>
        You should execute this utility during load to not over tune.<br><br>
        <code>m_cost</code> is optional and the safe minimal value of <code>32768</code> would be chosen, if empty.<br>
        <code>p_cost</code> is optional and rauthy will utilize all threads it can see, if empty.<br>
    </p>

    <Argon2Utility/>

    <div style="height: 20px"></div>
</div>

<style>
    .container {
        margin: 0 10px;
    }

    .label {
        width: 90px;
    }

    code {
        padding: 0 3px;
        background: var(--col-inact);
    }

    .valRow {
        display: flex;
    }
</style>
