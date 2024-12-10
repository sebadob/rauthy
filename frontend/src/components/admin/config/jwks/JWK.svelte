<script>
    import {fade} from 'svelte/transition';

    let { jwk } = $props();

    let isHover = $state(false);

    function renderType() {
        if (jwk.kty === 'OKP') {
            return jwk.crv;
        }

        if (jwk.kty === 'RSA') {
            let bits;
            const len = jwk.n.length;
            if (len > 600) {
                bits = 4096;
            } else if (len > 400) {
                bits = 3072;
            } else {
                bits = 2048;
            }

            return `RSA ${bits}`;
        }

        return 'unknown type';
    }

</script>

<div
        role="none"
        class="cert"
        onmouseenter={() => isHover = true}
        onmouseleave={() => isHover = false}
>
    <div class="data font-mono">
        {jwk.kid}
    </div>

    <div class="data right">
        {renderType()}
    </div>

    {#if isHover}
        <div class="details" transition:fade|global>
            <div class="detailsInner">
        <textarea
                name={jwk.kid}
                class="font-mono text"
                rows={10}
                cols={40}
                value={JSON.stringify(jwk, null, 2)}
        ></textarea>
            </div>
        </div>
    {/if}
</div>

<style>
    .cert {
        display: flex;
        justify-content: space-between;
        max-width: 420px;
        margin: 5px;
        padding: 5px;
        border: 1px solid var(--col-acnt);
        border-radius: 5px;
        box-shadow: 0 0 5px rgba(128, 128, 128, .2);
    }

    .cert:hover {
        background-color: var(--col-gmid);
    }

    .data {
        flex: 1;
    }

    .details {
        position: relative;
    }

    .detailsInner {
        display: block;
        position: absolute;
        top: -157px;
        left: 5px;
        z-index: 1;
    }

    .right {
        text-align: right;
    }

    .text {
        border-radius: 5px;
    }
</style>
