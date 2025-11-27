<script lang="ts">
    import type { ThemeCss } from '$api/types/themes.ts';
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
    import RauthyLogo from '$lib/RauthyLogo.svelte';

    let {
        logoUrl,
        borderRadius,
        theme,
        typ,
    }: {
        logoUrl: string;
        borderRadius: string;
        theme: ThemeCss;
        typ: 'light' | 'dark';
    } = $props();

    let isLoading = $state(false);
    let showDefault = $state(false);

    $effect(() => {
        if (logoUrl) {
            showDefault = false;
        }
    });

    $effect(() => {
        if (isLoading) {
            setTimeout(() => {
                isLoading = false;
            }, 2000);
        }
    });
</script>

<div
    aria-label="Preview: All components inside are only for theme and colors preview and have no effect or interaction"
    style:--border-radius={borderRadius}
    style:--text={theme.text.join(' ')}
    style:--text-high={theme.text_high.join(' ')}
    style:--bg={theme.bg.join(' ')}
    style:--bg-high={theme.bg_high.join(' ')}
    style:--action={theme.action.join(' ')}
    style:--accent={theme.accent.join(' ')}
    style:--error={theme.error.join(' ')}
    style:--btn-text={theme.btn_text}
    style:--theme-sun={theme.theme_sun}
    style:--theme-moon={theme.theme_moon}
>
    <div class="inner">
        <div class="container">
            <div class="logo">
                {#if showDefault}
                    <RauthyLogo
                        width="100%"
                        show={typ}
                    />
                {:else}
                    <img
                        src={logoUrl}
                        alt="Client Logo"
                        width="100%"
                        height="100%"
                        onerror={() => (showDefault = true)}
                    />
                {/if}
            </div>

            <h3>Header</h3>
            <p>Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam</p>
            <p><span>--accent-color</span></p>

            <Input
                label="Preview"
                placeholder="Preview"
                width="12.5rem"
            />

            <div class="btn">
                <div>
                    <Button
                        level={1}
                        {isLoading}
                        onclick={() => (isLoading = true)}>Button</Button
                    >
                    <br />
                    <Button
                        level={2}
                        {isLoading}
                        onclick={() => (isLoading = true)}>Button</Button
                    >
                    <br />
                    <Button
                        level={3}
                        {isLoading}
                        onclick={() => (isLoading = true)}>Button</Button
                    >
                    <br />
                </div>

                <div>
                    <Button
                        level={-1}
                        {isLoading}
                        onclick={() => (isLoading = true)}>Button</Button
                    >
                    <br />
                    <Button
                        level={-2}
                        {isLoading}
                        onclick={() => (isLoading = true)}>Button</Button
                    >
                    <br />
                    <Button
                        level={-3}
                        {isLoading}
                        onclick={() => (isLoading = true)}>Button</Button
                    >
                </div>
            </div>

            <a
                href={window.location.href}
                onclick={ev => ev.preventDefault()}>Link</a
            >

            <br />
            <ThemeSwitch />
        </div>
    </div>
</div>

<style>
    h3 {
        color: hsl(var(--text-high));
    }

    span {
        color: hsl(var(--accent));
    }

    .btn {
        display: grid;
        grid-template-columns: 1fr 1fr;
    }

    .btn > div {
        width: 5rem;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .container {
        padding: 1rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
    }

    .inner {
        padding: 1rem;
        width: 17rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        background: hsl(var(--bg));
        color: hsl(var(--text));
    }

    .logo {
        margin: 0 0.25rem;
        width: 84px;
        height: 84px;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
