<script lang="ts">
    import {onMount} from "svelte";
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import type {UserAttrValueRequest, UserAttrValuesUpdateRequest, UserResponse} from "$api/types/user.ts";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {fetchGet, fetchPut} from "$api/fetch";
    import type {
        UserAttrConfigResponse,
        UserAttrConfigValueResponse,
        UserAttrValueResponse, UserAttrValuesResponse,
    } from "$api/types/user_attrs.ts";

    let {
        user,
        onSave,
    }: {
        user: UserResponse,
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);

    interface AttrValue {
        name: string,
        desc?: string,
        value: string,
    }

    let attrsAll: UserAttrConfigValueResponse[] = $state([]);
    let attrsUser: UserAttrValueResponse[] = $state([]);

    let attrsCombined: AttrValue[] = $state([]);

    onMount(() => {
        fetchAllAttrs();
    })

    $effect(() => {
        fetchUserAttr();
    });

    $effect(() => {
        attrsCombined = attrsAll.map(a => {
            let v: AttrValue = {
                name: a.name,
                desc: a.desc,
                value: attrsUser.find(au => au.key === a.name)?.value || '',
            };
            return v;
        });
    });

    async function fetchAllAttrs() {
        let res = await fetchGet<UserAttrConfigResponse>('/auth/v1/users/attr');
        if (res.body) {
            attrsAll = res.body.values.toSorted((a, b) => a.name.localeCompare(b.name));
        } else {
            err = res.error?.message || 'Error fetching attrs';
        }
    }

    async function fetchUserAttr() {
        let res = await fetchGet<UserAttrValuesResponse>(`/auth/v1/users/${user.id}/attr`);
        if (res.body) {
            attrsUser = res.body.values;
        } else {
            err = res.error?.message || 'Error fetching users attrs';
        }
    }

    async function onSubmit() {
        err = '';

        let userKeys = attrsUser.map(a => a.key);
        let payload: UserAttrValuesUpdateRequest = {
            values: attrsCombined
                .filter(a => a.value.trim() || userKeys.includes(a.name))
                .map(a => {
                    let v: UserAttrValueRequest = {
                        key: a.name,
                        value: a.value.trim(),
                    };
                    return v;
                })
        };

        let res = await fetchPut(`/auth/v1/users/${user.id}/attr`, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
        }

        onSave();
    }

</script>

<div class="container">
    {#if attrsCombined.length === 0}
        <p>{ta.common.noEntries}</p>
    {:else}
        <p>{ta.users.descAttr}</p>

        {#each attrsCombined as a}
            <Input
                    bind:value={a.value}
                    autocomplete="off"
                    label={a.name}
                    placeholder={a.desc || 'JSON Value'}
            />
        {/each}

        <div class="flex gap-05">
            <Button onclick={onSubmit}>
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck/>
            {/if}
        </div>
    {/if}
</div>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}


<style>
    .container {
        max-width: 467pt;
    }
</style>
