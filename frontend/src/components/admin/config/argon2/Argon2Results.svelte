<script>
    export let res = [];
</script>

<div class="container">
    <div class="row">
        <div class="columnWide">
            Time Taken (ms)
        </div>

        <div class="column">
            t_cost
        </div>

        <div class="column">
            m_cost
        </div>

        <div class="column">
            p_cost
        </div>
    </div>

    {#each res as r}
        <div class="row">
            <div class="columnWide">
                {r.time_taken}
            </div>

            <div class="column">
                {r.t_cost}
            </div>

            <div class="column">
                {r.m_cost}
            </div>

            <div class="column">
                {r.p_cost}
            </div>
        </div>
    {/each}

    <div style="height: 20px"></div>
</div>

<style>
    .container {
        margin: 0 20px;
    }

    .column, .columnWide {
        padding: 5px 2px;
        border-bottom: 1px solid var(--col-gmid);
    }

    .column {
        width: 90px;
    }

    .columnWide {
        width: 130px;
    }

    .row {
        display: flex;
    }

    .row:hover {
        background: var(--col-gmid);
    }
</style>
