<script>
    import ClientBrandingPreviewBtn from "./ClientBrandingPreviewBtn.svelte";
    import ClientBrandingPreviewInput from "./ClientBrandingPreviewInput.svelte";
    import ClientLogo from "../../ClientLogo.svelte";

    let {clientId = $bindable(), colors = $bindable(), isLoading} = $props();

    let errEmail = $state('This is an E-Mail error');
    let errPwd = $state('This is a password error');

</script>

<div class="preview font-label">
    <b>PREVIEW</b>
</div>
<div
        class="container"
        style:color={colors.text}
        style:border="1px solid {colors.gmid}"
        style:background={colors.bg}
>
    <div class="head">
        <div class="logo">
            {#if !isLoading}
                <ClientLogo {clientId}/>
            {/if}
        </div>
    </div>

    <div class="name">
        <h2>Client Name</h2>
    </div>

    <ClientBrandingPreviewInput
            type="email"
            name="rauthyEmail"
            placeholder="E-Mail"
            error={errEmail}
            {colors}
    >
        E-MAIL
    </ClientBrandingPreviewInput>

    <ClientBrandingPreviewInput
            password
            name="rauthyPassword"
            placeholder="Password"
            error={errPwd}
            {colors}
    >
        E-MAIL
    </ClientBrandingPreviewInput>

    <div class="btn">
        <ClientBrandingPreviewBtn {colors} level={1}/>
        <ClientBrandingPreviewBtn {colors} level={2}/>
        <ClientBrandingPreviewBtn {colors} level={3}/>
    </div>

</div>

<style>
    .btn {
        margin: 5px 0;
    }

    .container {
        width: 300px;
        height: 455px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        padding: 20px;
        border-radius: 5px;
        box-shadow: 5px 5px 5px rgba(128, 128, 128, .1);
    }

    .head {
        display: flex;
        justify-content: space-between;
        padding-right: 35px;
    }

    .logo {
        width: 84px;
        height: 84px;
    }

    .name {
        margin: -10px 5px 0 5px;
    }

    .preview {
        margin: 0 5px;
        font-size: .85rem;
    }
</style>
