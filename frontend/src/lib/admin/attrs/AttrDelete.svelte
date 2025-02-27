<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {fetchDelete} from "$api/fetch.ts";
    import type {UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";

    let {
        attr,
        onSave,
    }: {
        attr: UserAttrConfigValueResponse,
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');

    async function onSubmit() {
        err = '';

        let res = await fetchDelete(`/auth/v1/users/attr/${attr.name}`);
        if (res.error) {
            err = res.error.message;
        } else {
            onSave();
        }
    }
</script>

<p>{ta.attrs.delete1}</p>

<Button level={-1} onclick={onSubmit}>
    {t.common.delete}
</Button>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}
