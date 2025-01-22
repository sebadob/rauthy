<script>
    import {slide} from 'svelte/transition';
    import IconChevronRight from "$lib/icons/IconChevronRight.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {TPL_ERROR_TEXT, TPL_STATUS_CODE} from "../utils/constants.js";

    let t = useI18n();

    let showDetails = $state(false);
    let statusCode = $state('');
    let errorText = $state('');
    let errorDetails = $state('');

</script>

<Template id={TPL_STATUS_CODE} bind:value={statusCode}/>
<Template id={TPL_ERROR_TEXT} bind:value={errorText}/>

<Main>
    <ContentCenter>
        <div class="flex-col">
            <h1>{statusCode}</h1>
            <br>
            <p>{errorText}</p>

            {#if errorDetails}
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
                        {errorDetails}
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
