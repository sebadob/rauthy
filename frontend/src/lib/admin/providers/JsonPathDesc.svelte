<script lang="ts">
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let ta = useI18nAdmin();
</script>

<details>
    <summary>{ta.providers.config.sections?.claimHelp ?? 'JSONPath syntax and examples'}</summary>
    <div class="ui-muted-copy">
        <p>{@html ta.providers.config.jsonPath.p2}</p>
        <ul>
            <li>{@html ta.providers.config.jsonPath.p3}</li>
            <li>{@html ta.providers.config.jsonPath.p4}</li>
            <li>{@html ta.providers.config.jsonPath.p5}</li>
            <li>{@html ta.providers.config.jsonPath.p6}</li>
        </ul>
    </div>
</details>

<style>
    summary {
        width: fit-content;
        cursor: pointer;
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
    }

    details > div {
        margin-top: 0.5rem;
    }

    ul {
        padding-inline-start: 1.5rem;
    }
</style>
