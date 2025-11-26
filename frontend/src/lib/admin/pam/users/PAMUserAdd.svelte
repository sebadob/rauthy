<script lang="ts">
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import Form from '$lib/form/Form.svelte';
	import Button from '$lib/button/Button.svelte';
	import { useI18n } from '$state/i18n.svelte';
	import Input from '$lib/form/Input.svelte';
	import type {
		PamGroupResponse,
		PamUnlinkedEmailsResponse,
		PamUserCreateRequest,
		PamUserResponse,
	} from '$api/types/pam';
	import { PATTERN_LINUX_USERNAME } from '$utils/patterns';
	import Options from '$lib/Options.svelte';
	import { fetchGet, fetchPost } from '$api/fetch';
	import { onMount } from 'svelte';
	import LabeledValue from '$lib/LabeledValue.svelte';

	let {
		onCreate,
	}: {
		onCreate: (user: PamUserResponse) => void;
	} = $props();

	let t = useI18n();
	let ta = useI18nAdmin();

	let ref: undefined | HTMLInputElement = $state();

	let err = $state('');
	let options: string[] = $state([]);
	let user: PamUserCreateRequest = $state({
		username: '',
		email: '-',
	});

	onMount(() => {
		fetchUnlinkedEmails();
	});

	$effect(() => {
		requestAnimationFrame(() => {
			ref?.focus();
		});
	});

	async function fetchUnlinkedEmails() {
		let res = await fetchGet<PamUnlinkedEmailsResponse>('/auth/v1/pam/emails_unlinked');
		if (res.body) {
			options = res.body.emails;
		} else {
			console.error(res.error);
		}
	}

	async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
		if (user.email === '-') {
			return;
		}
		err = '';

		let res = await fetchPost<PamUserResponse>(form.action, user);
		if (res.error) {
			err = res.error.message;
		} else if (res.body) {
			onCreate(res.body);
		}
	}
</script>

<div class="container">
	<h1>{ta.pam.addUser}</h1>

	<Form
		action="/auth/v1/pam/users"
		{onSubmit}
	>
		<Input
			bind:ref
			label={ta.common.name}
			placeholder={ta.common.name}
			bind:value={user.username}
			required
			pattern={PATTERN_LINUX_USERNAME}
			width="min(22rem, 100%)"
		/>

		<LabeledValue label={ta.pam.userEmail}>
			<div class="flex">
				<Options
					ariaLabel={ta.pam.userEmail}
					{options}
					bind:value={user.email}
					borderless
					withSearch
					maxHeight="20rem"
				/>
				<span class="req">*</span>
			</div>
		</LabeledValue>

		<div class="caution">
			<b class="err">{ta.common.caution}:</b>
			<p>{ta.pam.usernameNewDesc}</p>
		</div>

		<div class="btn">
			<Button type="submit">
				{t.common.save}
			</Button>
		</div>

		{#if err}
			<div clasS="err">
				{err}
			</div>
		{/if}
	</Form>
</div>

<style>
	h1 {
		font-size: 1.1rem;
	}

	.btn {
		margin-top: 1rem;
	}

	.caution {
		margin: 1rem 0 2rem 0;
	}

	.container {
		width: 20rem;
		text-align: left;
	}

	.req {
		color: hsl(var(--error));
		font-size: 0.7rem;
	}
</style>
