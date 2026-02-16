<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import { PATTERN_CLIENT_ID, PATTERN_CLIENT_NAME } from '$utils/patterns';
    import Form from '$lib5/form/Form.svelte';
    import { fetchPost } from '$api/fetch';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import type { ClientResponse, NewClientRequest } from '$api/types/clients.ts';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import InputTags from '$lib5/form/InputTags.svelte';

    let {
        clients,
        onSave,
    }: {
        clients: ClientResponse[];
        onSave: (id: string) => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let ref: undefined | HTMLInputElement = $state();
    let err = $state('');

    let id = $state('');
    let name = $state('');
    let confidential = $state(true);
    let redirectURIs: string[] = $state([]);
    let postLogoutRedirectURIs: string[] = $state([]);

    $effect(() => {
        requestAnimationFrame(() => {
            ref?.focus();
        });
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (clients.find(c => c.id === id)) {
            err = ta.common.nameExistsAlready;
            return;
        }
        err = '';

        let payload: NewClientRequest = {
            id,
            name: name || undefined,
            confidential,
            redirect_uris: redirectURIs,
            post_logout_redirect_uris:
                postLogoutRedirectURIs.length > 0 ? postLogoutRedirectURIs : undefined,
        };
        let res = await fetchPost<ClientResponse>(form.action, payload);
        if (res.body) {
            onSave(res.body.id);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<div class="container">
    <Form action="/auth/v1/clients" {onSubmit}>
        <Input
            bind:ref
            bind:value={id}
            autocomplete="off"
            label="Client ID"
            placeholder="Client ID"
            required
            pattern={PATTERN_CLIENT_ID}
        />
        <Input
            bind:value={name}
            autocomplete="off"
            label={ta.clients.name}
            placeholder={ta.clients.name}
            pattern={PATTERN_CLIENT_NAME}
        />

        <InputCheckbox ariaLabel={ta.clients.confidential} bind:checked={confidential}>
            {ta.clients.confidential}
        </InputCheckbox>

        <p>{@html ta.clients.descUri}</p>
        <InputTags
            typ="url"
            bind:values={redirectURIs}
            label="Redirect URIs"
            errMsg={ta.validation.uri}
        />
        <InputTags
            typ="url"
            bind:values={postLogoutRedirectURIs}
            label="Post Logout Redirect URIs"
            errMsg={ta.validation.uri}
        />

        <Button type="submit">
            {t.common.save}
        </Button>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </Form>
</div>

<style>
    p {
        line-height: 1.25rem;
        margin-bottom: -0.5rem;
    }

    .container {
        width: min(25rem, 100dvw - 1.75rem);
        min-height: 7rem;
        text-align: left;
    }
</style>
