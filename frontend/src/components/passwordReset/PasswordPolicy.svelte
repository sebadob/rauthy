<script>
    export let t = {
        passwordPolicy: {
            passwordPolicy: "Password Policy",
            lengthMin: "Length min",
            lengthMax: "Length max",
            lowercaseMin: "Lowercase letters min",
            uppercaseMin: "Uppercase letters min",
            digitsMin: "Digits min",
            specialMin: "Special characters min",
            notRecent: "Not one of last recent passwords",
        }
    };
    export let policy = {};
    export let password = '';
    export let accepted = false;

    let errPolicy = [false, false, false, false, false, false];

    const checkPolicy = (pwd) => {
        if (!policy) {
            return false;
        }

        let pErr = [false, false, false, false, false, false];
        let err = false;

        // check min and max length
        if (pwd.length < policy.length_min) {
            pErr[0] = true;
            err = true;
        }
        if (pwd.length > policy.length_max) {
            pErr[1] = true;
            err = true;
        }

        // get character counts
        let counts = [0, 0, 0, 0];
        for (let i = 0; i < pwd.length; i++) {
            let code = pwd.charCodeAt(i);

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

        if (policy.include_lower_case !== -1 && policy.include_lower_case > counts[0]) {
            pErr[2] = true;
            err = true;
        }
        if (policy.include_upper_case !== -1 && policy.include_upper_case > counts[1]) {
            pErr[3] = true;
            err = true;
        }
        if (policy.include_digits !== -1 && policy.include_digits > counts[2]) {
            pErr[4] = true;
            err = true;
        }
        if (policy.include_special !== -1 && policy.include_special > counts[3]) {
            pErr[5] = true;
            err = true;
        }

        errPolicy = pErr;
        accepted = !err;
    }

    $: if (password) {
        checkPolicy(password);
    }
</script>

{#if policy}
    <div class="policyContainer">
      <span style="margin-left: 20px">
        <b>{t.passwordPolicy.passwordPolicy}</b>
      </span>

        <ul>
            <li class="li" class:policyErr={!!errPolicy[0]}>
                {t.passwordPolicy.lengthMin}
                {policy.length_min}
            </li>

            <li class="li" class:policyErr={!!errPolicy[1]}>
                {t.passwordPolicy.lengthMax}
                {policy?.length_max}
            </li>

            {#if -1 !== policy.include_lower_case}
                <li class="li" class:policyErr={!!errPolicy[2]}>
                    {t.passwordPolicy.lowercaseMin}
                    {policy?.include_lower_case}
                </li>
            {/if}

            {#if -1 !== policy.include_upper_case}
                <li class="li" class:policyErr={!!errPolicy[3]}>
                    {t.passwordPolicy.uppercaseMin}
                    {policy?.include_upper_case}
                </li>
            {/if}

            {#if -1 !== policy.include_digits}
                <li class="li" class:policyErr={!!errPolicy[4]}>
                    {t.passwordPolicy.digitsMin}
                    {policy?.include_digits}
                </li>
            {/if}

            {#if -1 !== policy.include_special}
                <li class="li" class:policyErr={!!errPolicy[5]}>
                    {t.passwordPolicy.specialMin}
                    {policy?.include_special}
                </li>
            {/if}

            {#if -1 !== policy.not_recently_used}
                <li class="li">
                    {t.passwordPolicy.notRecent}
                    {policy?.not_recently_used}
                </li>
            {/if}
        </ul>
    </div>
{/if}

<style>
    .li {
        width: 100%;
    }

    .policyContainer {
        margin-left: -10px;
    }

    .policyErr {
        color: var(--col-err);
    }
</style>
