<script lang="ts">
    import {genKey} from "$utils/helpers.ts";

    let {providerId}: { providerId: string } = $props();
</script>

<!--
We append a random param here to force the browser to do a refetch, if the logo has been changed.
This is only needed for the Admin UI, when we upload a new version of the image.
Without this (unused) query param, the browser would never reload the <img> tag.
TODO handle caching of these properly, maybe with ETAGs or simply expire after 12 hours
-->
<img src="{`/auth/v1/providers/${providerId}/img?${genKey()}}`}" width={20} height={20} alt="No Logo Available"/>
