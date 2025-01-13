<script>
    import {slide} from 'svelte/transition';
    import IconChevronRight from "$lib/icons/IconChevronRight.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import BrowserCheck from "./BrowserCheck.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";

    let t = $state();
    let showDetails = $state(false);

</script>

<Main>
    <ContentCenter>
        <WithI18n bind:t content="error">
            <div class="flex-col">
                <h1>{t.error}</h1>
                <br>
                <p>{t.errorText}</p>

                {#if t.detailsText}
                    <div
                            role="button"
                            tabindex="0"
                            class="showDetails"
                            onclick={() => showDetails = !showDetails}
                            onkeypress={() => showDetails = !showDetails}
                    >
                        {t.details}
                        <div
                                class="chevron"
                                style:margin-top={showDetails ? '' : '-5px'}
                                style:transform={showDetails ? 'rotate(90deg)' : 'rotate(180deg)'}
                        >
                            <IconChevronRight
                                    color="var(--col-act2)"
                                    width={16}
                            />
                        </div>
                    </div>

                    {#if showDetails}
                        <div transition:slide class="details">
                            {t.detailsText}
                        </div>
                    {/if}
                {/if}

                <LangSelector absolute/>
            </div>
        </WithI18n>
    </ContentCenter>
</Main>

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
