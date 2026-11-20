<script lang="ts">
    let { title, count }: { title: string; count: number } = $props();
</script>

<div class="admin-list-heading">
    <h2>{title}</h2>
    <span>{count}</span>
</div>

<style>
    .admin-list-heading {
        display: flex;
        align-items: center;
        gap: 10px;
        margin: 0 0 24px;
    }
    .admin-list-heading h2 {
        margin: 0;
        font-size: 22px;
        line-height: 28px;
        font-weight: 600;
        letter-spacing: -0.03em;
        color: var(--ui-text);
    }
    .admin-list-heading > span {
        display: grid;
        place-items: center;
        min-width: 24px;
        height: 24px;
        padding: 0 6px;
        border-radius: var(--border-radius);
        background: var(--ui-container);
        font-size: 12px;
        font-weight: 500;
    }
</style>
