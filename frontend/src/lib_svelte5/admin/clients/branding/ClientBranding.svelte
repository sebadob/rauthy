<script lang="ts">
    import type {ClientResponse} from "$api/types/clients.ts";
    import {fetchDelete, fetchPost} from "$api/fetch.ts";
    import type {ThemeRequestResponse} from "$api/types/themes.ts";
    import Form from "$lib5/form/Form.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import Button from "$lib5/button/Button.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Input from "$lib5/form/Input.svelte";
    import {PATTERN_CSS_VALUE_LOOSE} from "$utils/patterns.ts";
    import BrandingMode from "$lib5/admin/clients/branding/BrandingMode.svelte";

    let {
        client,
    }: {
        client: ClientResponse,
    } = $props();

    const inputWidth = "13rem";

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let theme: ThemeRequestResponse | undefined = $state();
    const tabs = ['light', 'dark'];
    let editTheme = $state(tabs[0]);

    let url = $derived(`/auth/v1/theme/${client.id}`);

    $effect(() => {
        fetchTheme();
    });

    async function fetchTheme() {
        let res = await fetchPost<ThemeRequestResponse>(url);
        // let res = await fetchGet(`/auth/v1/theme/${client.id}`, 'json', 'reload');
        if (res.body) {
            if (res.body.client_id === 'rauthy') {
                console.log('using Rauthy default scheme');
                // TODO maybe show an indicator that it's the default scheme?
            }
            theme = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (!theme) {
            console.error('theme is undefined');
            return;
        }

        let payload = theme;
        // this could be `rauthy` the default has been used before
        payload.client_id = client.id;
        console.log('payload', payload);
    }

    async function reset() {
        let res = await fetchDelete(url);
        if (res.error) {
            err = res.error.message;
        } else {
            await fetchTheme();
        }
    }
</script>

<div class="container">
    {#if theme}
        <Form action={url} {onSubmit}>
            <Input
                    label="border-radius"
                    placeholder="border-radius"
                    errMsg={ta.validation.css}
                    width={inputWidth}
                    bind:value={theme.border_radius}
                    required
                    pattern={PATTERN_CSS_VALUE_LOOSE}
            />

            <h2>Light</h2>
            <BrandingMode bind:values={theme.light} {inputWidth}/>

            <h2>Dark</h2>
            <BrandingMode bind:values={theme.dark} {inputWidth}/>

            <!-- TODO: switchable preview in compact mode only -->
            <!--        <div class="flex">-->
            <!--            <Tabs {tabs} bind:selected={editTheme}/>-->
            <!--        </div>-->

            <div class="buttons">
                <Button type="submit">
                    {t.common.save}
                </Button>
                <Button level={-2} onclick={reset}>
                    {ta.common.reset}
                </Button>
            </div>

            {#if err}
                <div class="err">
                    {err}
                </div>
            {/if}
        </Form>
    {/if}
</div>

<style>
    h2 {
        font-size: 1rem;
    }

    .buttons {
        margin: 1rem 0;
    }

    /*.container {*/
    /*    border: 1px solid yellow;*/
    /*}*/
</style>
