<script lang="ts">
	import { page } from '$app/stores';
	import { beforeNavigate } from '$app/navigation';
	import type { LayoutData } from './$types';
	import Notifier from '$lib/ui/Notifier.svelte';

	let profileOpen = false;
	let mobileMenuOpen = false;

	export let data: LayoutData;

	let links = new Map();
	links.set('dashboard', `/admin/${$page.params.subdomain}/dashboard`);
	links.set('posts', `/admin/${$page.params.subdomain}/posts`);
	links.set('clipping', `/admin/${$page.params.subdomain}/clipping`);
	links.set('media', `/admin/${$page.params.subdomain}/media`);
	links.set('pages', `/admin/${$page.params.subdomain}/pages`);
	links.set('profile', `/admin/${$page.params.subdomain}/profile`);
	links.set('settings', `/admin/${$page.params.subdomain}/settings`);

	links.set('logout', `/admin/${$page.params.subdomain}/login?/logout`);

	beforeNavigate(async () => {
		profileOpen = false;
		mobileMenuOpen = false;
	});
</script>

<style lang="postcss">
	.current {
		@apply bg-gray-900 text-white;
	}
</style>

<svelte:head>
	<title>{$page.params.subdomain} | {$page.data.pageTitle}</title>
</svelte:head>
<div class="min-h-screen bg-gray-100">
	<nav class="bg-slate-800">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="flex h-16 items-center justify-between">
				<div class="flex items-center">
					<div class="flex text-white">
						<div class="text-white">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								width="24"
								height="24">
								<path fill="none" d="M0 0H24V24H0z" />
								<path
									fill="currentColor"
									d="M18.328 4.256l-1.423 1.423c-3.138-2.442-7.677-2.22-10.562.664-2.374 2.374-2.944 5.868-1.71 8.78l2.417-2.417c-.213-1.503.258-3.085 1.414-4.242 1.71-1.71 4.352-1.922 6.293-.636l-1.464 1.464c-1.115-.532-2.49-.337-3.414.587-.924.923-1.12 2.299-.587 3.414l-6.45 6.45c-.034-3.5-.591-4.812-.788-6.702-.301-2.894.657-5.894 2.875-8.112 3.666-3.666 9.471-3.89 13.4-.673zm2.83.002c.034 3.5.591 4.811.788 6.701.301 2.894-.657 5.894-2.875 8.112-3.666 3.666-9.471 3.89-13.4.673l1.424-1.423c3.138 2.442 7.677 2.22 10.562-.664 2.374-2.374 2.944-5.868 1.71-8.78l-2.417 2.417c.213 1.503-.258 3.085-1.414 4.242-1.71 1.71-4.352 1.922-6.293.636l1.464-1.464c1.115.532 2.49.337 3.414-.587.924-.923 1.12-2.299.587-3.414l6.45-6.45z" />
							</svg>
						</div>
						<div class="ml-1">/ {$page.params.subdomain}</div>
					</div>
					<div class="hidden md:block">
						<div class="ml-10 flex items-baseline space-x-4">
							<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
							<a
								href={links.get('dashboard')}
								class="text-gray-300 hover:bg-gray-700 rounded-md px-3 py-2 text-sm font-medium"
								class:current={$page.url.pathname.includes('dashboard')}
								aria-current="page">
								Dashboard
							</a>

							<a
								href={links.get('posts')}
								class:current={$page.url.pathname.includes('posts')}
								class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
								Posts
							</a>

							<a
								href={links.get('pages')}
								class:current={$page.url.pathname.includes('pages')}
								class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
								Pages
							</a>

							<a
								href={links.get('clipping')}
								class:current={$page.url.pathname.includes('clipping')}
								class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
								Clipping
							</a>

							<a
								href={links.get('media')}
								class:current={$page.url.pathname.includes('media')}
								class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
								Media Library
							</a>

							<a
								href={links.get('analytics')}
								class:current={$page.url.pathname.includes('analytics')}
								class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
								Analytics
							</a>
						</div>
					</div>
				</div>
				<div class="hidden md:block">
					<div class="ml-4 flex items-center md:ml-6">
						<button
							type="button"
							class="rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
							<span class="sr-only">View notifications</span>
							<svg
								class="h-6 w-6"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
							</svg>
						</button>

						<!-- Profile dropdown -->
						<div class="relative ml-3">
							<div>
								<button
									type="button"
									class="flex max-w-xs items-center rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
									id="user-menu-button"
									on:click={() => {
										profileOpen = !profileOpen;
									}}
									aria-expanded="false"
									aria-haspopup="true">
									<span class="sr-only">Open user menu</span>
									<img
										class="h-8 w-8 rounded-full"
										src={`https://www.gravatar.com/avatar/${data.gravatar}`}
										alt="" />
								</button>
							</div>

							<!--
                  Dropdown menu, show/hide based on menu state.
  
                  Entering: "transition ease-out duration-100"
                    From: "transform opacity-0 scale-95"
                    To: "transform opacity-100 scale-100"
                  Leaving: "transition ease-in duration-75"
                    From: "transform opacity-100 scale-100"
                    To: "transform opacity-0 scale-95"
                -->
							<div
								class:hidden={!profileOpen}
								class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
								role="menu"
								aria-orientation="vertical"
								aria-labelledby="user-menu-button"
								tabindex="-1">
								<!-- Active: "bg-gray-100", Not Active: "" -->
								<a
									href="#"
									class="block px-4 py-2 text-sm text-gray-700"
									role="menuitem"
									tabindex="-1"
									id="user-menu-item-0">
									Your Profile
								</a>

								<a
									href="#"
									class="block px-4 py-2 text-sm text-gray-700"
									role="menuitem"
									tabindex="-1"
									id="user-menu-item-1">
									Settings
								</a>

								<a
									href={links.get('logout')}
									class="block px-4 py-2 text-sm text-gray-700"
									role="menuitem"
									tabindex="-1"
									id="user-menu-item-2">
									Sign out
								</a>
							</div>
						</div>
					</div>
				</div>
				<div class="-mr-2 flex md:hidden">
					<!-- Mobile menu button -->
					<button
						type="button"
						class="inline-flex items-center justify-center rounded-md bg-gray-800 p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
						on:click={() => {
							mobileMenuOpen = !mobileMenuOpen;
						}}
						aria-controls="mobile-menu"
						aria-expanded="false">
						<span class="sr-only">Open main menu</span>
						<!-- Menu open: "hidden", Menu closed: "block" -->
						<svg
							class="block h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
						</svg>
						<!-- Menu open: "block", Menu closed: "hidden" -->
						<svg
							class="hidden h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>
			</div>
		</div>

		<!-- Mobile menu, show/hide based on menu state. -->
		<div class:hidden={!mobileMenuOpen} class="md:hidden" id="mobile-menu">
			<div class="space-y-1 px-2 pt-2 pb-3 sm:px-3">
				<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
				<a
					href={links.get('dashboard')}
					class:current={$page.url.pathname.includes('dashboard')}
					on:click={() => {
						mobileMenuOpen = false;
					}}
					class="text-gray-300 hover:bg-gray-700 block rounded-md px-3 py-2 text-base font-medium"
					aria-current="page">
					Dashboard
				</a>

				<a
					href={links.get('posts')}
					on:click={() => {
						mobileMenuOpen = false;
					}}
					class:current={$page.url.pathname.includes('posts')}
					class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">
					Posts
				</a>

				<a
					href={links.get('pages')}
					on:click={() => {
						mobileMenuOpen = false;
					}}
					class:current={$page.url.pathname.includes('pages')}
					class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">
					Pages
				</a>

				<a
					href={links.get('clipping')}
					on:click={() => {
						mobileMenuOpen = false;
					}}
					class:current={$page.url.pathname.includes('clipping')}
					class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">
					Clipping
				</a>

				<a
					href={links.get('media')}
					on:click={() => {
						mobileMenuOpen = false;
					}}
					class:current={$page.url.pathname.includes('media')}
					class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">
					Media Library
				</a>

				<a
					href={links.get('analytics')}
					on:click={() => {
						mobileMenuOpen = false;
					}}
					class:current={$page.url.pathname.includes('analytics')}
					class="text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium">
					Analytics
				</a>
			</div>
			<div class="border-t border-gray-700 pt-4 pb-3">
				<div class="flex items-center px-5">
					<div class="flex-shrink-0">
						<img
							class="h-10 w-10 rounded-full"
							src={`https://www.gravatar.com/avatar/${data.gravatar}`}
							alt="" />
					</div>
					<div class="ml-3">
						{#if data.display_name}
							<div class="text-base font-medium leading-none text-white">
								{data.display_name}
							</div>
						{/if}
						<div class="text-sm font-medium leading-none text-gray-400">
							{data.email}
						</div>
					</div>
					<button
						type="button"
						class="ml-auto flex-shrink-0 rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
						<span class="sr-only">View notifications</span>
						<svg
							class="h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
						</svg>
					</button>
				</div>
				<div class="mt-3 space-y-1 px-2">
					<a
						href="#"
						class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white">
						Your Profile
					</a>

					<a
						href="#"
						class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white">
						Settings
					</a>

					<a
						href={links.get('logout')}
						class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white">
						Sign out
					</a>
				</div>
			</div>
		</div>
	</nav>
	<div class="py-6 sm:px-6 lg:px-8">
		<slot />
	</div>
</div>
<Notifier />
