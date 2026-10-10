<script lang="ts">
    import LabeledValue from '$lib/LabeledValue.svelte';
    import Options from '$lib/Options.svelte';
    import Template from '$lib5/Template.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { TPL_PASSKEY_REG_TYPE } from '$utils/constants';

    let {
        residentKey = $bindable(false),
    }: {
        residentKey: boolean;
    } = $props();

    let t = useI18n();

    // `webauthn.passkey_reg_type`: space-separated, the first one is the default
    let regTypes = $state('default resident_key');
    // [Default, Resident Key] -> the index matches `t.account.passkeys.types`
    let allowed = $derived.by(() => {
        let idx = regTypes
            .split(' ')
            .filter(typ => typ === 'default' || typ === 'resident_key')
            .map(typ => (typ === 'resident_key' ? 1 : 0));
        return idx.length > 0 ? idx : [0, 1];
    });
    let options = $derived(allowed.map(i => t.account.passkeys.types[i]));
    let selected = $state(t.account.passkeys.types[0]);

    $effect(() => {
        // the Template is resolved after mount -> start with the configured default
        selected = options[0];
    });

    $effect(() => {
        residentKey = selected === t.account.passkeys.types[1];
    });
</script>

<Template id={TPL_PASSKEY_REG_TYPE} bind:value={regTypes} />

<LabeledValue label={t.account.passkeys.type}>
    {#if options.length > 1}
        <Options {options} bind:value={selected} ariaLabel={t.account.passkeys.type} />
    {:else}
        {selected}
    {/if}
    <div class="desc" class:rk={residentKey}>
        <p>{t.account.passkeys.typesDesc[residentKey ? 1 : 0]}</p>
        {#if residentKey}
            <p class="rkWarn">{t.account.passkeys.rkWarning}</p>
        {/if}
    </div>
</LabeledValue>

<style>
    .desc {
        max-width: 30rem;
        margin: 0.5rem 0;
        padding: 0.5rem 0.75rem;
        border-left: 3px solid hsl(var(--accent));
        border-radius: var(--border-radius);
        background: hsla(var(--accent) / 0.1);
    }

    .desc.rk {
        border-left-color: hsl(var(--error));
        background: hsla(var(--error) / 0.1);
    }

    .desc p {
        margin: 0;
    }

    .rkWarn {
        margin-top: 0.5rem !important;
        color: hsl(var(--error));
        font-size: 0.9rem;
    }
</style>
