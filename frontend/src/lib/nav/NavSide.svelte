<script lang="ts">
	import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
	import Button from '$lib5/button/Button.svelte';
	import LangSelector from '$lib5/LangSelector.svelte';
	import { page } from '$app/state';
	import AppVersion from '../AppVersion.svelte';
	import RauthyLogo from '$lib5/RauthyLogo.svelte';
	import NavLink from '$lib5/nav/NavLink.svelte';
	import IconArrowPathSquare from '$icons/IconArrowPathSquare.svelte';
	import IconUserGroup from '$icons/IconUserGroup.svelte';
	import IconDocText from '$icons/IconDocText.svelte';
	import IconOffice from '$icons/IconOffice.svelte';
	import IconShieldCheck from '$icons/IconShieldCheck.svelte';
	import IconUser from '$icons/IconUser.svelte';
	import IconId from '$icons/IconId.svelte';
	import IconBell from '$icons/IconBell.svelte';
	import IconStop from '$icons/IconStop.svelte';
	import IconKey from '$icons/IconKey.svelte';
	import IconCloud from '$icons/IconCloud.svelte';
	import IconWrenchScrew from '$icons/IconWrenchScrew.svelte';
	import IconBookOpen from '$icons/IconBookOpen.svelte';
	import IconCheckBadge from '$icons/IconCheckBadge.svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import IconLogout from '$icons/IconLogout.svelte';
	import { redirectToLogout } from '$utils/helpers';
	import Tooltip from '$lib5/Tooltip.svelte';
	import IconCommandLine from '$icons/IconCommandLine.svelte';

	let ta = useI18nAdmin();

	let timeout: undefined | number;

	let optsButtons = $state(false);
	let params = $state('');
	let compact = $state(false);
	let collapsed = $state(false);

	let innerWidth: undefined | number = $state();

	// cannot be derived because of static pre-rendering
	$effect(() => {
		params = page.url.search;
	});

	$effect(() => {
		if (!innerWidth) {
			return;
		}
		let to_collapse = (page.url && innerWidth < 600) || innerWidth < 800;
		let be_compact = innerWidth < 1280;

		// we wait for an animation frame to avoid display errors in some browsers
		requestAnimationFrame(() => {
			collapsed = to_collapse;
			compact = be_compact;
		});
	});

	function showOpts() {
		if (timeout) {
			clearTimeout(timeout);
		}
		optsButtons = true;
	}

	function hideOpts() {
		setTimeout(() => {
			optsButtons = false;
		}, 1000);
	}

	function toggleCollapsed() {
		collapsed = !collapsed;
	}

	function toggleCompact() {
		compact = !compact;
	}
</script>

<svelte:window bind:innerWidth />

<nav
	id="mainNav"
	aria-hidden={collapsed}
	onmouseenter={showOpts}
	onmouseleave={hideOpts}
	data-compact={compact}
>
	<div>
		<div>
			<div class="relative">
				<div
					class="absolute flex space-between navmod"
					aria-hidden={!collapsed && !optsButtons}
				>
					<div>
						<Button
							ariaControls="mainNav"
							invisible
							onclick={toggleCollapsed}
						>
							<svg
								aria-expanded={!collapsed}
								class="inner"
								viewBox="0 0 24 24"
								stroke="currentColor"
								stroke-width={2}
								width="1.5rem"
								opacity={0.9}
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
								/>
							</svg>
						</Button>
					</div>

					{#if !collapsed}
						<div>
							<Button
								invisible
								onclick={toggleCompact}
							>
								<span class="inner">
									<IconArrowPathSquare />
								</span>
							</Button>
						</div>
					{/if}
				</div>
			</div>

			{#if !collapsed}
				<div class="logo">
					<RauthyLogo width={compact ? '3rem' : '7rem'} />
				</div>

				<div
					class="menu"
					style:margin-top={compact ? '.5rem' : '1.5rem'}
				>
					<NavLink
						{compact}
						{params}
						route="/users"
					>
						{#snippet icon(width: string)}
							<IconUser {width} />
						{/snippet}
						{ta.nav.users}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/clients"
					>
						{#snippet icon(width: string)}
							<IconOffice {width} />
						{/snippet}
						{ta.nav.clients}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/roles"
					>
						{#snippet icon(width: string)}
							<IconCheckBadge {width} />
						{/snippet}
						{ta.nav.roles}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/groups"
					>
						{#snippet icon(width: string)}
							<IconUserGroup {width} />
						{/snippet}
						{ta.nav.groups}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/attributes"
					>
						{#snippet icon(width: string)}
							<IconDocText {width} />
						{/snippet}
						{ta.nav.attributes}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/scopes"
					>
						{#snippet icon(width: string)}
							<IconId {width} />
						{/snippet}
						{ta.nav.scopes}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/pam"
					>
						{#snippet icon(width: string)}
							<IconCommandLine {width} />
						{/snippet}
						PAM
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/sessions"
					>
						{#snippet icon(width: string)}
							<IconShieldCheck {width} />
						{/snippet}
						{ta.nav.sessions}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/events"
					>
						{#snippet icon(width: string)}
							<IconBell {width} />
						{/snippet}
						{ta.nav.events}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/blacklist"
					>
						{#snippet icon(width: string)}
							<IconStop
								{width}
								color="currentColor"
							/>
						{/snippet}
						{ta.nav.blacklist}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/api_keys"
					>
						{#snippet icon(width: string)}
							<IconKey {width} />
						{/snippet}
						{ta.nav.apiKeys}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/providers"
					>
						{#snippet icon(width: string)}
							<IconCloud {width} />
						{/snippet}
						{ta.nav.providers}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/config/policy"
						highlightIncludes="/config/"
					>
						{#snippet icon(width: string)}
							<IconWrenchScrew {width} />
						{/snippet}
						{ta.nav.config}
					</NavLink>

					<NavLink
						{compact}
						{params}
						route="/docs"
					>
						{#snippet icon(width: string)}
							<IconBookOpen {width} />
						{/snippet}
						{ta.nav.docs}
					</NavLink>
				</div>
			{/if}
		</div>

		{#if !collapsed}
			<div class="bottom">
				{#snippet toLogout()}
					<div class="logout">
						<Button
							invisible
							onclick={() => redirectToLogout()}
						>
							<Tooltip text="Logout">
								<IconLogout />
							</Tooltip>
						</Button>
					</div>
				{/snippet}

				{#if compact}
					{@render toLogout()}
					<div class="theme">
						<ThemeSwitch />
					</div>
					<LangSelector openTop />
				{:else}
					<div class="flex gap-05">
						<ThemeSwitch />
						<LangSelector openTop />
						{@render toLogout()}
					</div>
					<div class="version">
						<AppVersion />
					</div>
				{/if}
			</div>
		{/if}
	</div>
</nav>

<style>
	nav {
		width: 12.5rem;
		height: 100dvh;
		background: hsl(var(--bg));
		z-index: 101;
		transition: width 150ms;
	}

	@media (max-width: 800px) {
		nav {
			position: absolute;
			top: 0;
			left: 0;
		}
	}

	nav > div {
		width: 100%;
		height: 100%;
		padding: 1rem 1rem 0.5rem 1rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		background: hsla(var(--bg-high) / 0.25);
		transition: all 150ms;
	}

	nav[data-compact='true'] {
		width: 4rem;
	}

	nav[data-compact='true'] > div {
		padding: 0.5rem 0.2rem 0 0.2rem;
		text-align: center;
		line-height: 0.8rem;
	}

	nav[aria-hidden='true'] {
		width: 0;
	}

	nav[aria-hidden='true'] > div {
		padding: 0;
	}

	.bottom {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		line-height: 1.5rem;
	}

	nav[data-compact='true'] .theme {
		margin-bottom: -0.5rem;
	}

	.logo {
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--border-radius);
		overflow: clip;
	}

	.logout {
		margin-bottom: -0.25rem;
	}

	.menu {
		max-height: 85dvh;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		overflow-y: auto;
	}

	nav[data-compact='true'] .menu {
		max-height: 80dvh;
	}

	.navmod {
		opacity: 0;
		margin: -1rem 0 0 -1rem;
		width: calc(100% + 1.8rem);
		transition: all 150ms ease-in-out;
	}

	nav[aria-hidden='true'] .navmod {
		opacity: 1;
		margin: 0 0 0 0.25rem;
	}

	nav[data-compact='true'] .navmod {
		margin: -0.2rem 0 0 -0.2rem;
		width: calc(100% + 0.4rem);
	}

	.navmod[aria-hidden='false'],
	.navmod:hover {
		opacity: 1;
	}

	.navmod .inner {
		color: hsla(var(--text) / 0.5);
		transition: all 150ms ease-in-out;
	}

	nav[aria-hidden='true'] .navmod .inner {
		color: hsla(var(--text) / 0.7);
	}

	nav[aria-hidden='true'] .navmod .inner:hover,
	.navmod .inner:hover {
		color: hsl(var(--action));
	}

	.version {
		margin: 0 0 -0.5rem -0.5rem;
	}
</style>
