<script>
    import {slide} from 'svelte/transition';
    import IconChevronRight from "$lib/icons/IconChevronRight.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";

    let t = useI18n();
    let showDetails = $state(false);

    // TODO:
    // The `details` and `detailsText` cannot be translated statically.
    // -> implement a mechanism to fetch them either from the body or query params

</script>

<Main>
    <ContentCenter>
        <div class="flex-col">
            <h1>{t.error.error}</h1>
            <br>
            <p>{t.error.errorText}</p>

            {#if t.error.detailsText}
                <div
                        role="button"
                        tabindex="0"
                        class="showDetails"
                        onclick={() => showDetails = !showDetails}
                        onkeypress={() => showDetails = !showDetails}
                >
                    {t.error.details}
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
                        {t.error.detailsText}
                    </div>
                {/if}
            {/if}

            <LangSelector absolute/>
        </div>
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
