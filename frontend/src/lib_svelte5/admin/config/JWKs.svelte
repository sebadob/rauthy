<script lang="ts">
    import {onMount} from "svelte";
    import Button from "$lib5/Button.svelte";
    import {postRotateJwk} from "$utils/dataFetchingAdmin.js";
    import {fetchGet} from "$api/fetch.ts";
    import type {JWKSCerts, JWKSPublicKeyCerts} from "$api/types/oidc.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import Expandable from "$lib5/Expandable.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
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

        let res = await postRotateJwk();
        if (res.ok) {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
            await fetchCerts();
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

</script>

<ContentAdmin>
    <h2>Json Web Keys</h2>
    <p>
        These are the Json Web Keys (JWKs) used for token singing.
    </p>

    <p>
        You can rotate them and generate a full new set. Depending on your deployment, this could take a few
        seconds.<br>
        New tokens will always be signed with the new / latest ones. The old keys will be cleaned up automatically,
        when
        there cannot be a token anymore that used the old key to not
        break any current token validation.
    </p>

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

    <div class="btn">
        <Button onclick={onSubmit} {isLoading}>
            {ta.jwks.rotateKeys}
        </Button>
    </div>

    {#if success}
        <IconCheck/>
    {/if}

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</ContentAdmin>

<style>
    .btn {
        margin-top: 1rem;
    }

    .n {
        max-width: 35rem;
        word-break: break-all;
    }
</style>
