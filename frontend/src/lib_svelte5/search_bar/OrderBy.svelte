<script lang="ts">
    import IconBarsArrowDown from "$lib/icons/IconBarsArrowDown.svelte";
    import IconBarsArrowUp from "$lib/icons/IconBarsArrowUp.svelte";
    import Button from "$lib5/Button.svelte";
    import Tooltip from "$lib5/Tooltip.svelte";
    import Options from "$lib5/Options.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

    let {
        onChange,
        options = [],
        borderless,
        firstDirReverse,
    }: {
        onChange: (option: string, direction: 'up' | 'down') => void,
        options: string[],
        borderless?: boolean,
        firstDirReverse?: boolean,
    } = $props();

    let ta = useI18nAdmin();

    let selected = $state(options[0]);
    let direction: 'up' | 'down' = $state(firstDirReverse ? 'down' : 'up');

    $effect(() => {
        onChange(selected, direction);
    });
</script>

<div class="container">
    {#if options.length > 1}
        <Tooltip text={ta.search.orderBy}>
            <Options
                    ariaLabel={ta.search.orderBy}
                    {options}
                    bind:value={selected}
                    {borderless}
            />
        </Tooltip>
    {/if}

    <div class="btn">
        {#if direction === 'up'}
            <Button ariaLabel={ta.search.orderChangeToDesc} invisible onclick={() => direction = 'down'}>
                <Tooltip text={ta.search.orderChangeToDesc}>
                    <IconBarsArrowUp/>
                </Tooltip>
            </Button>
        {:else}
            <Button ariaLabel={ta.search.orderChangeToAsc} invisible onclick={() => direction = 'up'}>
                <Tooltip text={ta.search.orderChangeToAsc}>
                    <IconBarsArrowDown/>
                </Tooltip>
            </Button>
        {/if}
    </div>
</div>

<style>
    .btn {
        margin: .25rem 1rem -.25rem .5rem;
    }

    .container {
        display: flex;
        align-items: center;
    }
</style>
