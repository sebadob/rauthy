<script>
    import IconStop from "$lib/icons/IconStop.svelte";

    /** @type {boolean} */
    export let showModal;

    /** @type {HTMLDialogElement} */
    let dialog;

    $: if (dialog && showModal) dialog.showModal();
</script>

<!-- According to MDN docs, a dialog element must not have a tabindex -->
<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
        bind:this={dialog}
        on:close={() => (showModal = false)}
        on:click|self={() => dialog.close()}
>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation>
        <div
                role="button"
                tabindex="0"
                class="close"
                on:click={() => dialog.close()}
        >
            <IconStop color="var(--col-err)" width={24}/>
        </div>
        <div class="inner">
            <!--
            Just make sure that whatever we have in here will be loaded / fetched lazy.
            There is no need to load resources like images if the dialog is closed anyway.
            -->
            {#if showModal}
                <slot></slot>
            {/if}
        </div>
    </div>
</dialog>

<style>
    dialog {
        border-radius: 3px;
        border: none;
        padding: 0;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.3);
    }

    dialog > div {
        padding: 1rem;
    }

    dialog > div > div {
        position: relative;
    }

    dialog[open] {
        animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    @keyframes zoom {
        from {
            transform: scale(0.95);
        }
        to {
            transform: scale(1);
        }
    }

    dialog[open]::backdrop {
        animation: fade 0.2s ease-out;
    }

    @keyframes fade {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .close {
        margin: -1rem -1rem 0 0;
        cursor: pointer;
        text-align: right;
    }

    .inner {
        margin-top: -.5rem;
    }
</style>
