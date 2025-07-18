<script lang="ts">
    import type {PamGroupResponse} from "$api/types/pam";
    import LabeledValue from "$lib/LabeledValue.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";

    let {
        group,
    }: {
        group: PamGroupResponse,
    } = $props();

    let ta = useI18nAdmin();

</script>

<h1>{group.name}</h1>

<LabeledValue label="ID">
    {group.id}
</LabeledValue>
<LabeledValue label={ta.pam.groupName}>
    {group.name}
</LabeledValue>
<LabeledValue label={ta.pam.groupType}>
    {group.typ}
</LabeledValue>

{#if group.name === 'wheel-rauthy'}
    <p>{ta.pam.wheelGroupDesc}</p>
{:else if group.typ === 'host'}
    <p>{ta.pam.hostGroupDesc}</p>
{:else if group.typ === 'user'}
    <p>{ta.pam.userGroupDesc}</p>
{/if}

<!--
TODO:
 - immutable groups can never be deleted
 - host groups should only be deletable when they have no host assigned
 - generic + local groups should be deletable at any time
-->
