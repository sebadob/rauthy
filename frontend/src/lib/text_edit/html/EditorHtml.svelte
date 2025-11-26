<script lang="ts">
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import ToolbarIconHtml from '$lib/text_edit/html/tools/ToolbarIconHtml.svelte';
	import IconH1 from '$icons/editor/IconH1.svelte';
	import IconH2 from '$icons/editor/IconH2.svelte';
	import IconH3 from '$icons/editor/IconH3.svelte';
	import IconBold from '$icons/editor/IconBold.svelte';
	import IconItalic from '$icons/editor/IconItalic.svelte';
	import IconStrikeThrough from '$icons/editor/IconStrikeThrough.svelte';
	import IconListBullet from '$icons/editor/IconListBullet.svelte';
	import IconListNumbered from '$icons/editor/IconListNumbered.svelte';
	import ToolIconParagraph from '$lib/text_edit/html/tools/ToolIconParagraph.svelte';
	import IconCode from '$icons/editor/IconCode.svelte';
	import IconQuote from '$icons/editor/IconQuote.svelte';
	import ToolIconLink from '$lib/text_edit/html/tools/ToolIconLink.svelte';
	import ToolIconRemoveFmt from '$lib/text_edit/html/tools/ToolIconRemoveFmt.svelte';

	let {
		content = $bindable(),
		height = '50rem',
		onblur,
	}: {
		content: string;
		height?: string;
		onblur?: () => void;
	} = $props();

	const iconSize = '1.3rem';

	let ta = useI18nAdmin();

	let ref: undefined | HTMLDivElement = $state();
</script>

<div
	role="none"
	class="editor"
	style:height
	onclick={() => ref?.focus()}
>
	<div class="toolbar">
		<ToolbarIconHtml
			{ref}
			title={ta.editor.heading1}
			tagName="h1"
		>
			<IconH1 width={iconSize} />
		</ToolbarIconHtml>
		<ToolbarIconHtml
			{ref}
			title={ta.editor.heading2}
			tagName="h2"
		>
			<IconH2 width={iconSize} />
		</ToolbarIconHtml>
		<ToolbarIconHtml
			{ref}
			title={ta.editor.heading1}
			tagName="h3"
		>
			<IconH3 width={iconSize} />
		</ToolbarIconHtml>

		<ToolbarIconHtml
			{ref}
			title={ta.editor.bold}
			tagName="strong"
		>
			<IconBold width={iconSize} />
		</ToolbarIconHtml>

		<ToolbarIconHtml
			{ref}
			title={ta.editor.italic}
			tagName="em"
		>
			<IconItalic width={iconSize} />
		</ToolbarIconHtml>

		<ToolbarIconHtml
			{ref}
			title={ta.editor.strikeThrough}
			tagName="s"
		>
			<IconStrikeThrough width={iconSize} />
		</ToolbarIconHtml>

		<ToolbarIconHtml
			{ref}
			title={ta.editor.listBullet}
			tagName="li"
			tagNameParent="ul"
		>
			<IconListBullet width={iconSize} />
		</ToolbarIconHtml>
		<ToolbarIconHtml
			{ref}
			title={ta.editor.listNumbered}
			tagName="li"
			tagNameParent="ol"
		>
			<IconListNumbered width={iconSize} />
		</ToolbarIconHtml>
		<!-- TODO checkbox list -> needs multiple mandatory parents -->

		<ToolIconParagraph
			{ref}
			{iconSize}
		/>

		<ToolbarIconHtml
			{ref}
			title={ta.editor.code}
			tagName="code"
		>
			<IconCode width={iconSize} />
		</ToolbarIconHtml>
		<ToolbarIconHtml
			{ref}
			title={ta.editor.quote}
			tagName="blockquote"
		>
			<IconQuote width={iconSize} />
		</ToolbarIconHtml>

		<ToolIconLink
			{ref}
			{iconSize}
		/>
		<ToolIconRemoveFmt
			{ref}
			{iconSize}
		/>
	</div>

	<div class="content theme-light">
		<div
			role="textbox"
			tabindex="0"
			bind:this={ref}
			class="edit theme-light"
			aria-label={ta.editor.textArea}
			bind:innerHTML={content}
			contenteditable
			spellcheck="false"
			{onblur}
		></div>
	</div>
</div>

<style>
	.content {
		flex: 1;
		background-color: hsl(var(--bg));
		overflow-y: auto;
	}

	.edit {
		padding: 1rem 1rem 0 1rem;
		display: block;
		outline: none;
		overflow-y: auto;
		color: hsl(var(--text));
		background-color: hsl(var(--bg));
	}

	.editor {
		display: flex;
		flex-direction: column;
		border: 1px solid hsl(var(--bg-high));
		border-radius: var(--border-radius);
		overflow-y: auto;
	}

	.toolbar {
		padding: 0.25rem 0.5rem 0 0.5rem;
		display: flex;
		align-items: center;
		gap: 0 0.25rem;
		flex-wrap: wrap;
		border-bottom: 1px solid hsla(var(--bg-high), 0.8);
		cursor: default;
	}
</style>
