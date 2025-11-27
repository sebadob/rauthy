<script lang="ts">
	import { useI18nAdmin } from '$state/i18n_admin.svelte';

	let ta = useI18nAdmin();
</script>

<div class="desc">
	<h4>ID token claim mappings</h4>
	<p>{ta.providers.config.jsonPath.p1}</p>
	<p>{@html ta.providers.config.jsonPath.p2}</p>
	<p>{@html ta.providers.config.jsonPath.p3}</p>
	<p>{@html ta.providers.config.jsonPath.p4}</p>
	<p>{@html ta.providers.config.jsonPath.p5}</p>
	<p>{@html ta.providers.config.jsonPath.p6}</p>
</div>

<style>
	h4 {
		margin-bottom: 0.5rem;
	}

	.desc {
		display: flex;
		flex-direction: column;
		margin: 0.5rem 0;
	}

	.desc > p {
		margin: 0.2rem 0;
	}
</style>
