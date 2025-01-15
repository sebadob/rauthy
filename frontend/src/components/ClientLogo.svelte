<script>
    import {getKey} from "$lib/utils/helpers";

    let {clientId} = $props();
</script>

<!--
We append a random param here to force the browser to do a refetch, if the logo has been changed.
This is only needed for the Admin UI, when we upload a new version of the image.
Without this (unused) query param, the browser would never reload the <img> tag.
-->
<img src="{`/auth/v1/clients/${clientId}/logo?${getKey(6)}`}" alt="No Logo Available"/>