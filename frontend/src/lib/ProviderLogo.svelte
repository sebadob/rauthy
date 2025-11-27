<script lang="ts">
	import { genKey } from '$utils/helpers.ts';

	let { providerId }: { providerId: string } = $props();
</script>

<img
	src={`/auth/v1/providers/${providerId}/img?${genKey()}}`}
	width={20}
	height={20}
	alt="No Logo Available"
/>
