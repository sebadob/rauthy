<script lang="ts">
	import { PATTERN_URI } from '$utils/patterns';
	import Input from '$lib/form/Input.svelte';

	let {
		issuer = $bindable(),
		authorizationEndpoint = $bindable(),
		tokenEndpoint = $bindable(),
		userinfoEndpoint = $bindable(),
		inputWidth,
		disabled,
	}: {
		issuer: string;
		authorizationEndpoint: string;
		tokenEndpoint: string;
		userinfoEndpoint: string;
		inputWidth: string;
		disabled?: boolean;
	} = $props();
</script>

<Input
	bind:value={issuer}
	autocomplete="off"
	label="Issuer URL"
	placeholder="Issuer URL"
	required
	pattern={PATTERN_URI}
	width={inputWidth}
	{disabled}
/>
<Input
	typ="url"
	bind:value={authorizationEndpoint}
	autocomplete="off"
	label="Authorization Endpoint"
	placeholder="Authorization Endpoint"
	required
	pattern={PATTERN_URI}
	width={inputWidth}
	{disabled}
/>
<Input
	typ="url"
	bind:value={tokenEndpoint}
	autocomplete="off"
	label="Token Endpoint"
	placeholder="Token Endpoint"
	required
	pattern={PATTERN_URI}
	width={inputWidth}
	{disabled}
/>
<Input
	typ="url"
	bind:value={userinfoEndpoint}
	autocomplete="off"
	label="Userinfo Endpoint"
	placeholder="Userinfo Endpoint"
	required
	pattern={PATTERN_URI}
	width={inputWidth}
	{disabled}
/>
