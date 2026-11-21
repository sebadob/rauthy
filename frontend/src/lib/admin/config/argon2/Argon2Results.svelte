<script lang="ts">
    import type { PasswordHashTime } from '#api/types/password_hashing.ts';

    let {
        res,
    }: {
        res: PasswordHashTime[];
    } = $props();
</script>

<div class="results">
    <table>
        <thead>
            <tr>
                <th scope="col">ms</th>
                <th scope="col">t_cost</th>
                <th scope="col">m_cost</th>
                <th scope="col">p_cost</th>
            </tr>
        </thead>
        <tbody>
            {#each res as r}
                <tr>
                    <td>{r.time_taken}</td>
                    <td>{r.t_cost}</td>
                    <td>{r.m_cost}</td>
                    <td>{r.p_cost}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
    .results {
        overflow-x: auto;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
    }

    table {
        width: 100%;
        border-collapse: collapse;
        font-size: var(--ui-control-font-size);
        font-variant-numeric: tabular-nums;
    }

    th,
    td {
        padding: 0.5rem 0.75rem;
        text-align: end;
        white-space: nowrap;
    }

    th {
        color: var(--ui-muted);
        background: var(--ui-container);
    }

    td {
        border-top: 1px solid var(--ui-outline);
    }
</style>
