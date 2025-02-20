<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {fetchPut} from "$api/fetch.ts";
    import Form from "$lib5/form/Form.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import {PATTERN_CLIENT_NAME, PATTERN_CONTACT, PATTERN_ORIGIN, PATTERN_URI} from "$utils/patterns.ts";
    import {AuthFlowDeviceCode, type ClientResponse, type UpdateClientRequest} from "$api/types/clients.ts";
    import type {JwkKeyPairAlg} from "$api/types/oidc.ts";
    import InputTags from "$lib5/form/InputTags.svelte";
    import InputCheckbox from "$lib5/form/InputCheckbox.svelte";
    import SelectList from "$lib5/select_list/SelectList.svelte";
    import type {SelectItem} from "$lib5/select_list/props.ts";
    import {slide} from "svelte/transition";
    import Options from "$lib5/Options.svelte";
    import {untrack} from "svelte";

    let {
        client,
        clients,
        scopesAll,
        onSave,
    }: {
        client: ClientResponse,
        clients: ClientResponse[],
        scopesAll: string[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = 'min(20rem, calc(100dvw - .5rem))';
    let err = $state('');
    let success = $state(false);

    let id = $state(client.id);
    let name: string = $state(client.name || '');
    let enabled = $state(client.enabled);
    let confidential = $state(client.confidential);
    let uri: string = $state(client.client_uri || '');
    let contacts: string[] = $state(client.contacts ? Array.from(client.contacts) : []);
    let origins: string[] = $state(client.allowed_origins ? Array.from(client.allowed_origins) : []);
    let redirectURIs: string[] = $state(Array.from(client.redirect_uris));
    let postLogoutRedirectURIs: string[] = $state(client.post_logout_redirect_uris ? Array.from(client.post_logout_redirect_uris) : []);

    let flows = $state({
        authorizationCode: client.flows_enabled.includes('authorization_code'),
        clientCredentials: client.flows_enabled.includes('client_credentials'),
        password: client.flows_enabled.includes('password'),
        refreshToken: client.flows_enabled.includes('refresh_token'),
        deviceCode: client.flows_enabled.includes(AuthFlowDeviceCode),
    });

    const optionsAlgs: JwkKeyPairAlg[] = ['RS256', 'RS384', 'RS512', 'EdDSA'];
    let accessTokenAlg: JwkKeyPairAlg = $state(client.access_token_alg);
    let idTokenAlg: JwkKeyPairAlg = $state(client.id_token_alg);
    let tokenLifetime: string = $state(client.access_token_lifetime.toString());
    let authCodeLifetime: string = $state(client.auth_code_lifetime.toString());

    let scopes: SelectItem[] = $state(scopesAll.map(name => {
        let i: SelectItem = {
            name,
            selected: client.scopes.includes(name) || false,
        };
        return i;
    }));
    let defaultScopes: SelectItem[] = $state(scopesAll
        .map(name => {
            let i: SelectItem = {
                name,
                selected: client.default_scopes.includes(name) || false,
            };
            return i;
        }));
    let challenges = $state({
        plain: client.challenges?.includes('plain') || false,
        s256: client.challenges?.includes('S256') || false,
    });

    let forceMfa = $state(client.force_mfa);

    $effect(() => {
        if (client.id) {
            id = client.id;
            name = client.name || '';
            enabled = client.enabled;
            confidential = client.confidential;
            uri = client.client_uri || '';
            contacts = client.contacts ? Array.from(client.contacts) : [];
            origins = client.allowed_origins ? Array.from(client.allowed_origins) : [];
            redirectURIs = Array.from(client.redirect_uris);
            postLogoutRedirectURIs = client.post_logout_redirect_uris ? Array.from(client.post_logout_redirect_uris) : [];

            flows.authorizationCode = client.flows_enabled.includes('authorization_code');
            flows.clientCredentials = client.flows_enabled.includes('client_credentials');
            flows.password = client.flows_enabled.includes('password');
            flows.refreshToken = client.flows_enabled.includes('refresh_token');
            flows.deviceCode = client.flows_enabled.includes(AuthFlowDeviceCode);

            accessTokenAlg = client.access_token_alg;
            idTokenAlg = client.id_token_alg;
            tokenLifetime = client.access_token_lifetime.toString();
            authCodeLifetime = client.auth_code_lifetime.toString();

            scopes = scopesAll.map(name => {
                let i: SelectItem = {
                    name,
                    selected: client.scopes.includes(name) || false,
                };
                return i;
            });
            defaultScopes = scopesAll.map(name => {
                let i: SelectItem = {
                    name,
                    selected: client.default_scopes.includes(name) || false,
                };
                return i;
            });
            challenges.plain = client.challenges?.includes('plain') || false;
            challenges.s256 = client.challenges?.includes('S256') || false;
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        let payload: UpdateClientRequest = {
            id,
            name: name || undefined,
            enabled,
            confidential,
            allowed_origins: origins.length > 0 ? origins : undefined,
            redirect_uris: redirectURIs,
            post_logout_redirect_uris: postLogoutRedirectURIs.length > 0 ? postLogoutRedirectURIs : undefined,

            flows_enabled: [],
            access_token_alg: accessTokenAlg,
            id_token_alg: idTokenAlg,
            access_token_lifetime: Number.parseInt(tokenLifetime),
            auth_code_lifetime: Number.parseInt(authCodeLifetime),

            scopes: scopes.filter(s => s.selected).map(s => s.name),
            default_scopes: defaultScopes.filter(s => s.selected).map(s => s.name),
            challenges: undefined,

            force_mfa: forceMfa,
            client_uri: uri || undefined,
            contacts: contacts.length > 0 ? contacts : undefined,
        }

        if (flows.authorizationCode) {
            payload.flows_enabled.push('authorization_code');
        }
        if (flows.clientCredentials) {
            payload.flows_enabled.push('client_credentials');
        }
        if (flows.password) {
            payload.flows_enabled.push('password');
        }
        if (flows.refreshToken) {
            payload.flows_enabled.push('refresh_token');
        }
        if (flows.deviceCode) {
            payload.flows_enabled.push(AuthFlowDeviceCode);
        }

        if (challenges.plain) {
            payload.challenges = ['plain'];
        }
        if (challenges.s256) {
            if (payload.challenges) {
                payload.challenges.push('S256');
            } else {
                payload.challenges = ['S256'];
            }
        }

        console.log('payload', payload);

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            onSave();
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }
</script>

<Form action={`/auth/v1/clients/${client.id}`} {onSubmit}>
    <h5>{ta.common.information}</h5>

    <LabeledValue label="ID" mono>
        {client.id}
    </LabeledValue>

    <p class="desc">{ta.clients.descName}</p>
    <Input
            bind:value={name}
            autocomplete="off"
            label={ta.clients.name}
            placeholder={ta.clients.name}
            width={inputWidth}
            pattern={PATTERN_CLIENT_NAME}
    />

    <p class="desc">{ta.clients.descClientUri}</p>
    <Input
            typ="url"
            bind:value={uri}
            autocomplete="off"
            label="URI"
            placeholder="URI"
            width={inputWidth}
            pattern={PATTERN_URI}
    />
    <InputTags
            bind:values={contacts}
            label={ta.common.contact}
            pattern={PATTERN_CONTACT}
    />

    <div style:height=".5rem"></div>

    <h5>{ta.clients.config}</h5>

    <InputCheckbox ariaLabel={ta.common.enabled} bind:checked={enabled}>
        {ta.common.enabled}
    </InputCheckbox>
    <InputCheckbox ariaLabel={ta.clients.confidential} bind:checked={confidential}>
        {ta.clients.confidential}
    </InputCheckbox>
    <InputCheckbox ariaLabel={ta.clients.forceMfa} bind:checked={forceMfa}>
        {ta.clients.forceMfa}
    </InputCheckbox>

    <p class="mb-0">Authentication Flows</p>
    <InputCheckbox ariaLabel="authorization_code" bind:checked={flows.authorizationCode}>
        authorization_code
    </InputCheckbox>
    <InputCheckbox ariaLabel="urn:ietf:params:oauth:grant-type:device_code" bind:checked={flows.deviceCode}>
        device_code
    </InputCheckbox>
    <InputCheckbox ariaLabel="client_credentials" bind:checked={flows.clientCredentials}>
        client_credentials
    </InputCheckbox>
    <InputCheckbox ariaLabel="password" bind:checked={flows.password}>
        password
    </InputCheckbox>
    <InputCheckbox ariaLabel="refresh_token" bind:checked={flows.refreshToken}>
        refresh_token
    </InputCheckbox>

    <div style:height=".5rem"></div>
    <p class="desc">{ta.clients.descPKCE}</p>
    <p class="desc"><strong>{ta.clients.descPKCEEnforce}</strong></p>
    <InputCheckbox ariaLabel="PKCE plain" bind:checked={challenges.plain}>
        plain
    </InputCheckbox>
    <InputCheckbox ariaLabel="PKCE S256" bind:checked={challenges.s256}>
        S256
    </InputCheckbox>

    {#if !confidential && !challenges.plain && !challenges.s256}
        <div class="err" transition:slide={{duration: 150}}>
            {ta.clients.errConfidentialPKCE}
        </div>
    {/if}

    <div style:height=".5rem"></div>
    <p class="desc">{ta.clients.descOrigin}</p>
    <InputTags
            typ="url"
            bind:values={origins}
            label="Allowed Origins"
            errMsg={ta.validation.origin}
            pattern={PATTERN_ORIGIN}
    />

    <p class="desc">{@html ta.clients.descUri}</p>
    <InputTags
            typ="url"
            bind:values={redirectURIs}
            label="Redirect URIs"
            errMsg={ta.validation.uri}
            required={flows.authorizationCode}
            pattern={PATTERN_URI}
    />
    <InputTags
            typ="url"
            bind:values={postLogoutRedirectURIs}
            label="Post Logout Redirect URIs"
            errMsg={ta.validation.uri}
            pattern={PATTERN_URI}
    />

    <div style:height=".5rem"></div>
    <p class="desc">{@html ta.clients.scopes.desc}</p>
    <SelectList bind:items={scopes}>
        {ta.clients.scopes.allowed}
    </SelectList>
    <SelectList bind:items={defaultScopes}>
        {ta.clients.scopes.default}
    </SelectList>

    <div style:height=".75rem"></div>
    <p>{ta.clients.tokenLifetime.p1}</p>
    <Input
            typ="number"
            bind:value={tokenLifetime}
            autocomplete="off"
            label="Token Lifetime"
            placeholder="Token Lifetime"
            width={inputWidth}
            min="10"
            max="86400"
            errMsg="10 <= Token Lifetime <= 86400"
    />

    <div style:height=".5rem"></div>
    <p>{ta.clients.tokenLifetime.p2}</p>
    <p>{ta.clients.tokenLifetime.p3}</p>
    <LabeledValue label="Access Token Algorithm">
        <Options
                ariaLabel="Access Token Algorithm"
                options={optionsAlgs}
                bind:value={accessTokenAlg}
                borderless
        />
    </LabeledValue>
    <LabeledValue label="ID Token Algorithm">
        <Options
                ariaLabel="ID Token Algorithm"
                options={optionsAlgs}
                bind:value={idTokenAlg}
                borderless
        />
    </LabeledValue>

    <div style:height=".5rem"></div>
    <p>{ta.clients.descAuthCode}</p>
    <Input
            typ="number"
            bind:value={authCodeLifetime}
            autocomplete="off"
            label="Auth Code Lifetime"
            placeholder="Auth Code Lifetime"
            width={inputWidth}
            min="10"
            max="300"
            errMsg="10 <= Auth Code Lifetime <= 300"
    />

    <div class="flex gap-05" style:margin-top="1rem">
        <Button type="submit">
            {t.common.save}
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

    <div style:height="1rem"></div>
</Form>

<style>
    .desc {
        margin-bottom: -.25rem;
    }

    .mb-0 {
        margin-bottom: 0;
    }
</style>
