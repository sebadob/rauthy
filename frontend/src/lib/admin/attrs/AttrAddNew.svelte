<script lang="ts">
	import Button from '$lib5/button/Button.svelte';
	import Input from '$lib5/form/Input.svelte';
	import { PATTERN_ATTR, PATTERN_ATTR_DESC } from '$utils/patterns';
	import Form from '$lib5/form/Form.svelte';
	import { fetchPost } from '$api/fetch';
	import { useI18n } from '$state/i18n.svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import type {
		UserAttrConfigEntity,
		UserAttrConfigRequest,
		UserAttrConfigValueResponse,
	} from '$api/types/user_attrs.ts';

	let {
		attrs,
		onSave,
	}: {
		attrs: UserAttrConfigValueResponse[];
		onSave: (name: string) => void;
	} = $props();

	let t = useI18n();
	let ta = useI18nAdmin();

	let ref: undefined | HTMLInputElement = $state();

	let err = $state('');
	let name = $state('');
	let desc = $state('');

	$effect(() => {
		requestAnimationFrame(() => {
			ref?.focus();
		});
	});

	async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
		if (attrs.find(a => a.name === name)) {
			err = ta.common.nameExistsAlready;
			return;
		}
		err = '';

		let payload: UserAttrConfigRequest = {
			name,
			desc: desc || undefined,
		};
		let res = await fetchPost<UserAttrConfigEntity>(form.action, payload);
		if (res.body) {
			onSave(res.body.name);
		} else {
			err = res.error?.message || 'Error';
		}
	}
</script>

<div class="container">
	<Form
		action="/auth/v1/users/attr"
		{onSubmit}
	>
		<Input
			bind:ref
			bind:value={name}
			autocomplete="off"
			label={ta.attrs.name}
			placeholder={ta.attrs.name}
			required
			pattern={PATTERN_ATTR}
		/>
		<Input
			bind:value={desc}
			autocomplete="off"
			label={ta.attrs.desc}
			placeholder={ta.attrs.desc}
			pattern={PATTERN_ATTR_DESC}
		/>

		<Button type="submit">
			{t.common.save}
		</Button>

		{#if err}
			<div class="err">
				{err}
			</div>
		{/if}
	</Form>
</div>

<style>
	.container {
		min-height: 7rem;
		text-align: left;
	}
</style>
