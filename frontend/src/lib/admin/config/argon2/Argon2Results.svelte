<script lang="ts">
    import type { PasswordHashTime } from '$api/types/password_hashing.ts';

    let {
        res,
    }: {
        res: PasswordHashTime[];
    } = $props();
</script>

<div>
    <div class="row header font-label">
        <div>ms</div>
        <div>t_cost</div>
        <div>m_cost</div>
        <div>p_cost</div>
    </div>

    {#each res as r}
        <div class="row">
            <div>{r.time_taken}</div>
            <div>{r.t_cost}</div>
            <div>{r.m_cost}</div>
            <div>{r.p_cost}</div>
        </div>
    {/each}

    <div style="height: 1.5rem"></div>
</div>

<style>
    .header {
        color: hsl(var(--text-high));
    }

    .row {
        display: grid;
        grid-template-columns: 5.5rem 4rem 5rem 3rem;
    }

    .row > div {
        border-bottom: 1px solid hsl(var(--bg-high));
    }

    .row:hover {
        background: hsl(var(--accent));
    }
</style>
