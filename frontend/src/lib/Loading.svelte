<script lang="ts">
    import {fade} from 'svelte/transition';

    let {
        background = false,
        color = 'hsl(var(--text))',
        global = false,
        offset = 0,
    }: {
        background?: boolean;
        color?: string;
        global?: boolean;
        offset?: number;
    } = $props();

</script>

<div
        class="container"
        class:global
        class:local={!global}
        class:background
        transition:fade="{{ duration: 100 }}"
>
    <div class="loading" style="margin-top: {offset}px;">
        <div class="loading-1" style:background={color}></div>
        <div class="loading-2" style:background={color}></div>
        <div class="loading-3" style:background={color}></div>
    </div>
</div>

<style>
    .container {
        display: flex;
        flex: 1;
        justify-content: center;
        align-items: center;
        z-index: 1;
    }

    @keyframes fade-in {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    @keyframes fade-out {
        from {
            opacity: 1;
        }
        to {
            opacity: 0;
        }
    }

    .loading {
        width: 3.3rem;
        height: 1.1rem;
        aspect-ratio: 3;
        display: flex;
        justify-content: space-evenly;
        align-items: center;
    }

    .loading-1, .loading-2, .loading-3 {
        height: 66%;
        aspect-ratio: 1;
        border-radius: 50%;
        animation: 1s infinite alternate fade-in;
        opacity: 0;
    }

    .loading-2 {
        animation-delay: 333ms;
    }

    .loading-3 {
        animation-delay: 666ms;
    }

    .global {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
    }

    .local {
        min-width: 70px;
        max-width: 70px;
    }

    .background {
        background: rgba(127, 127, 127, .5);
    }
</style>
