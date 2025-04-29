<script lang="ts">
    import {onMount} from "svelte";
    import Button from "$lib5/button/Button.svelte";
    import {fetchGet, fetchPost} from "$api/fetch";
    import type {JWKSCerts, JWKSPublicKeyCerts} from "$api/types/oidc.ts";
    import Expandable from "$lib5/Expandable.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import IconCheck from "$icons/IconCheck.svelte";

    let ta = useI18nAdmin();

    let certs: JWKSPublicKeyCerts[] = $state([]);
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    onMount(() => {
        fetchCerts();
    });

    async function fetchCerts() {
        let res = await fetchGet<JWKSCerts>('/auth/v1/oidc/certs');
        if (res.body) {
            certs = res.body.keys;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        let res = await fetchPost(`/auth/v1/oidc/rotate_jwk`);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
            await fetchCerts();
        }

        isLoading = false;
    }

</script>

<h2>Json Web Keys</h2>
<p>{ta.jwks.p1}</p>
<p>{ta.jwks.p2}</p>
<p>{ta.jwks.p3}</p>

{#each certs as jwk (jwk.kid)}
    <Expandable>
        {#snippet summary()}
            <div class="font-mono">
                {jwk.alg}
                /
                {jwk.kid}
            </div>
        {/snippet}
        {#snippet details()}
            <LabeledValue label="Key ID" mono>
                {jwk.kid}
            </LabeledValue>
            <LabeledValue label={ta.jwks.type} mono>
                {jwk.kty}
            </LabeledValue>
            <LabeledValue label={ta.jwks.alg} mono>
                {jwk.alg}
            </LabeledValue>

            {#if jwk.crv}
                <LabeledValue label="Curve" mono>
                    {jwk.crv}
                </LabeledValue>
            {/if}
            {#if jwk.n}
                <LabeledValue label="n" mono>
                    <div class="n">
                        {jwk.n}
                    </div>
                </LabeledValue>
            {/if}
            {#if jwk.e}
                <LabeledValue label="e" mono>
                    {jwk.e}
                </LabeledValue>
            {/if}
            {#if jwk.x}
                <LabeledValue label="x" mono>
                    {jwk.x}
                </LabeledValue>
            {/if}
        {/snippet}
    </Expandable>
{/each}

<div class="btn flex gap-05">
    <Button onclick={onSubmit} {isLoading}>
        {ta.jwks.rotateKeys}
    </Button>

    {#if success}
        <IconCheck/>
    {/if}
</div>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<style>
    .btn {
        margin-top: 1rem;
    }

    .n {
        max-width: 35rem;
        word-break: break-all;
    }
</style>
