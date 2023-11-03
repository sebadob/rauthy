<script>
    import { slide } from 'svelte/transition';
    import IconChevronRight from "$lib/icons/IconChevronRight.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import BrowserCheck from "./BrowserCheck.svelte";
    import LangSelector from "$lib/LangSelector.svelte";

    let t;
    let showDetails = false;

</script>

<BrowserCheck>
    <WithI18n bind:t content="error">
        <h1>{t.error}</h1>
        <p>{t.errorText}</p>

        {#if t.detailsText}
            <div
                    role="button"
                    tabindex="0"
                    class="showDetails"
                    on:click={() => showDetails = !showDetails}
                    on:keypress={() => showDetails = !showDetails}
            >
                {t.details}
                <div
                        class="chevron"
                        style:margin-top={showDetails ? '' : '-5px'}
                        style:transform={showDetails ? 'rotate(90deg)' : 'rotate(180deg)'}
                >
                    <IconChevronRight
                            color="var(--col-act2)"
                            width=16
                    />
                </div>
            </div>

            {#if showDetails}
                <div transition:slide class="details">
                    {t.detailsText}
                </div>
            {/if}
        {/if}

        <LangSelector absolute />
    </WithI18n>
</BrowserCheck>

<style>
    .chevron {
        transition: all 250ms;
    }

    .showDetails {
        display: inline-flex;
        align-items: center;
        gap: .1rem;
        cursor: pointer;
        color: var(--col-act2);
    }

    .showDetails:hover {
        color: var(--col-act2a);
    }
</style>
