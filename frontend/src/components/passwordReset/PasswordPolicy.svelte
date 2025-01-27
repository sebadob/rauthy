<script lang="ts">
    import {useI18n} from "$state/i18n.svelte";
    import type {PasswordPolicyResponse} from "$api/types/password_policy.ts";

    let {
        policy,
        password,
        accepted = $bindable(false)
    }: {
        policy: PasswordPolicyResponse,
        password: string,
        accepted: boolean,
    } = $props();

    let t = useI18n();

    let errPolicy = $state([false, false, false, false, false, false]);

    $effect(() => {
        checkPolicy();
    });

    function checkPolicy() {
        if (!policy) {
            return false;
        }

        let pErr = [false, false, false, false, false, false];
        let err = false;

        if (password.length < policy.length_min) {
            pErr[0] = true;
            err = true;
        }
        if (password.length > policy.length_max) {
            pErr[1] = true;
            err = true;
        }

        // get character counts
        let counts = [0, 0, 0, 0];
        for (let i = 0; i < password.length; i++) {
            let code = password.charCodeAt(i);

            // is lowercase?
            if (code >= 97 && code <= 122) {
                counts[0] = counts[0] + 1;
                continue;
            }

            // is uppercase?
            if (code >= 65 && code <= 90) {
                counts[1] = counts[1] + 1;
                continue;
            }

            // is digit?
            if (code >= 48 && code <= 57) {
                counts[2] = counts[2] + 1;
                continue;
            }

            // can only be special
            counts[3] = counts[3] + 1;
        }

        if (policy.include_lower_case
            && policy.include_lower_case !== -1
            && policy.include_lower_case > counts[0]
        ) {
            pErr[2] = true;
            err = true;
        }
        if (policy.include_upper_case
            && policy.include_upper_case !== -1
            && policy.include_upper_case > counts[1]
        ) {
            pErr[3] = true;
            err = true;
        }
        if (policy.include_digits
            && policy.include_digits !== -1
            && policy.include_digits > counts[2]
        ) {
            pErr[4] = true;
            err = true;
        }
        if (policy.include_special
            && policy.include_special !== -1
            && policy.include_special > counts[3]
        ) {
            pErr[5] = true;
            err = true;
        }

        errPolicy = pErr;
        accepted = !err;
    }
</script>

{#if policy}
    <div class="policyContainer">
        <b>{t.passwordPolicy.passwordPolicy}</b>

        <ul>
            <li class="li" class:policyErr={!!errPolicy[0]}>
                {t.passwordPolicy.lengthMin}
                {policy.length_min}
            </li>

            <li class="li" class:policyErr={!!errPolicy[1]}>
                {t.passwordPolicy.lengthMax}
                {policy?.length_max || 0}
            </li>

            {#if -1 !== policy.include_lower_case}
                <li class="li" class:policyErr={!!errPolicy[2]}>
                    {t.passwordPolicy.lowercaseMin}
                    {policy?.include_lower_case || 0}
                </li>
            {/if}

            {#if -1 !== policy.include_upper_case}
                <li class="li" class:policyErr={!!errPolicy[3]}>
                    {t.passwordPolicy.uppercaseMin}
                    {policy?.include_upper_case || 0}
                </li>
            {/if}

            {#if -1 !== policy.include_digits}
                <li class="li" class:policyErr={!!errPolicy[4]}>
                    {t.passwordPolicy.digitsMin}
                    {policy?.include_digits || 0}
                </li>
            {/if}

            {#if -1 !== policy.include_special}
                <li class="li" class:policyErr={!!errPolicy[5]}>
                    {t.passwordPolicy.specialMin}
                    {policy?.include_special || 0}
                </li>
            {/if}

            {#if -1 !== policy.not_recently_used}
                <li class="li">
                    {t.passwordPolicy.notRecent}
                    {policy?.not_recently_used || 0}
                </li>
            {/if}
        </ul>
    </div>
{/if}

<style>
    b {
        margin-left: 1rem;
    }

    ul {
        margin-left: 1rem;
    }

    .li {
        margin-left: 1.5rem;
        width: 100%;
    }

    .policyContainer {
        margin-left: -.5rem;
        margin-bottom: 1rem;
    }

    .policyErr {
        color: hsl(var(--error));
    }
</style>
