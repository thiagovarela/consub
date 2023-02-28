<script lang="ts">
	import { page } from '$app/stores';
	import SidebarMenuItem from '$lib/ui/SidebarMenuItem.svelte';
	import Notifier from '$lib/ui/Notifier.svelte';

	let links = new Map();
	links.set('posts', `/admin/${$page.params.subdomain}/posts`);
	links.set('dashboard', `/admin/${$page.params.subdomain}/dashboard`);
	links.set('posts.categories', `/admin/${$page.params.subdomain}/posts/categories`);
	links.set('posts.drafts', `/admin/${$page.params.subdomain}/posts/drafts`);
	links.set('clipping', `/admin/${$page.params.subdomain}/clipping`);
	links.set('clipping.categories', `/admin/${$page.params.subdomain}/clipping/categories`);
	links.set('pages', `/admin/${$page.params.subdomain}/pages`);
	links.set('analytics', `/admin/${$page.params.subdomain}/pages`);

	let sidebarOpen = false;
</script>

<style lang="postcss">
	.menu-item {
		@apply flex w-full items-center rounded-md py-2 pl-11 pr-2 text-sm font-medium text-gray-600 hover:bg-gray-50 hover:text-gray-900;
	}

	.sidebar-back-open {
		@apply transition-opacity ease-linear duration-300 opacity-100;
	}

	.sidebar-back-closed {
		@apply transition-opacity ease-linear duration-300 opacity-0;
	}

	.sidebar-open {
		@apply transition ease-in-out duration-300 transform translate-x-0;
	}

	.sidebar-closed {
		@apply transition ease-in-out duration-300 transform -translate-x-full;
	}
</style>

<div class="flex">
	<!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
	<div class="relative z-40 md:hidden" role="dialog" aria-modal="true">
		<!--
        Off-canvas menu backdrop, show/hide based on off-canvas menu state.
  
        Entering: "transition-opacity ease-linear duration-300"
          From: "opacity-0"
          To: "opacity-100"
        Leaving: "transition-opacity ease-linear duration-300"
          From: "opacity-100"
          To: "opacity-0"
      -->
		<div
			class:sidebar-back-closed={!sidebarOpen}
			class:sidebar-back-open={sidebarOpen}
			class:hidden={!sidebarOpen}
			on:click={() => (sidebarOpen = false)}
			class="fixed inset-0 bg-gray-600 bg-opacity-75" />

		<div
			class="fixed inset-0 z-40 flex"
			class:sidebar-closed={!sidebarOpen}
			class:sidebar-open={sidebarOpen}>
			<!--
          Off-canvas menu, show/hide based on off-canvas menu state.
  
          Entering: "transition ease-in-out duration-300 transform"
            From: "-translate-x-full"
            To: "translate-x-0"
          Leaving: "transition ease-in-out duration-300 transform"
            From: "translate-x-0"
            To: "-translate-x-full"
        -->
			<div class="relative flex w-full max-w-xs flex-1 flex-col bg-white pt-5 pb-4">
				<!--
            Close button, show/hide based on off-canvas menu state.
  
            Entering: "ease-in-out duration-300"
              From: "opacity-0"
              To: "opacity-100"
            Leaving: "ease-in-out duration-300"
              From: "opacity-100"
              To: "opacity-0"
          -->
				<div class="absolute top-0 right-0 -mr-12 pt-2" class:hidden={!sidebarOpen}>
					<button
						on:click={() => {
							sidebarOpen = false;
						}}
						type="button"
						class="ml-1 flex h-10 w-10 items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white">
						<span class="sr-only">Close sidebar</span>
						<!-- Heroicon name: outline/x-mark -->
						<svg
							class="h-6 w-6 text-white"
							xmlns="http://www.w3.org/2000/svg"
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

				<div class="flex flex-shrink-0 items-center px-4">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						width="24"
						height="24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M18.328 4.256l-1.423 1.423c-3.138-2.442-7.677-2.22-10.562.664-2.374 2.374-2.944 5.868-1.71 8.78l2.417-2.417c-.213-1.503.258-3.085 1.414-4.242 1.71-1.71 4.352-1.922 6.293-.636l-1.464 1.464c-1.115-.532-2.49-.337-3.414.587-.924.923-1.12 2.299-.587 3.414l-6.45 6.45c-.034-3.5-.591-4.812-.788-6.702-.301-2.894.657-5.894 2.875-8.112 3.666-3.666 9.471-3.89 13.4-.673zm2.83.002c.034 3.5.591 4.811.788 6.701.301 2.894-.657 5.894-2.875 8.112-3.666 3.666-9.471 3.89-13.4.673l1.424-1.423c3.138 2.442 7.677 2.22 10.562-.664 2.374-2.374 2.944-5.868 1.71-8.78l-2.417 2.417c.213 1.503-.258 3.085-1.414 4.242-1.71 1.71-4.352 1.922-6.293.636l1.464-1.464c1.115.532 2.49.337 3.414-.587.924-.923 1.12-2.299.587-3.414l6.45-6.45z" />
					</svg>
					<span class="ml-2">Consub</span>
				</div>
				<div class="mt-5 h-0 flex-1 overflow-y-auto">
					<nav class="space-y-1 px-2">
						<SidebarMenuItem>
							<span slot="svg-icon">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									fill="currentColor"
									class="h-4 w-4"
									viewBox="0 0 24 24">
									<path fill="none" d="M0 0h24v24H0z" />
									<path
										d="M6.94 14.036c-.233.624-.43 1.2-.606 1.783.96-.697 2.101-1.139 3.418-1.304 2.513-.314 4.746-1.973 5.876-4.058l-1.456-1.455 1.413-1.415 1-1.001c.43-.43.915-1.224 1.428-2.368-5.593.867-9.018 4.292-11.074 9.818zM17 9.001L18 10c-1 3-4 6-8 6.5-2.669.334-4.336 2.167-5.002 5.5H3C4 16 6 2 21 2c-1 2.997-1.998 4.996-2.997 5.997L17 9.001z" />
								</svg>
							</span>
							<span slot="name">Posts</span>
							<span slot="items">
								<a href={links.get('posts')} class="menu-item">Posts</a>
								<a href={links.get('posts.categories')} class="menu-item">
									Categories
								</a>
							</span>
						</SidebarMenuItem>

						<SidebarMenuItem>
							<span slot="svg-icon">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									fill="currentColor"
									viewBox="0 0 24 24"
									class="h-4 w-4">
									<path fill="none" d="M0 0h24v24H0z" />
									<path
										d="M10 6c0 .732-.197 1.419-.54 2.01L12 10.585l6.728-6.728a2 2 0 0 1 2.828 0l-12.11 12.11a4 4 0 1 1-1.414-1.414L10.586 12 8.032 9.446A4 4 0 1 1 10 6zM8 6a2 2 0 1 0-4 0 2 2 0 0 0 4 0zm13.556 14.142a2 2 0 0 1-2.828 0l-5.317-5.316 1.415-1.415 6.73 6.731zM16 11h2v2h-2v-2zm4 0h2v2h-2v-2zM6 11h2v2H6v-2zm-4 0h2v2H2v-2zm4 9a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
								</svg>
							</span>
							<span slot="name">Clipping</span>
							<span slot="items">
								<a href={links.get('clipping')} class="menu-item">Clippings</a>

								<a href={links.get('clipping.categories')} class="menu-item">
									Categories
								</a>
							</span>
						</SidebarMenuItem>
					</nav>
				</div>
			</div>

			<div class="w-14 flex-shrink-0">
				<!-- Dummy element to force sidebar to shrink to fit close icon -->
			</div>
		</div>
	</div>

	<!-- Static sidebar for desktop -->
	<div class="hidden md:fixed md:inset-y-0 md:flex md:w-64 md:flex-col">
		<!-- Sidebar component, swap this element with another sidebar if you like -->
		<div class="flex flex-grow flex-col overflow-y-auto border-r border-gray-200 bg-white pt-5">
			<div class="flex flex-shrink-0 items-center px-4">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
					<path fill="none" d="M0 0H24V24H0z" />
					<path
						d="M18.328 4.256l-1.423 1.423c-3.138-2.442-7.677-2.22-10.562.664-2.374 2.374-2.944 5.868-1.71 8.78l2.417-2.417c-.213-1.503.258-3.085 1.414-4.242 1.71-1.71 4.352-1.922 6.293-.636l-1.464 1.464c-1.115-.532-2.49-.337-3.414.587-.924.923-1.12 2.299-.587 3.414l-6.45 6.45c-.034-3.5-.591-4.812-.788-6.702-.301-2.894.657-5.894 2.875-8.112 3.666-3.666 9.471-3.89 13.4-.673zm2.83.002c.034 3.5.591 4.811.788 6.701.301 2.894-.657 5.894-2.875 8.112-3.666 3.666-9.471 3.89-13.4.673l1.424-1.423c3.138 2.442 7.677 2.22 10.562-.664 2.374-2.374 2.944-5.868 1.71-8.78l-2.417 2.417c.213 1.503-.258 3.085-1.414 4.242-1.71 1.71-4.352 1.922-6.293.636l1.464-1.464c1.115.532 2.49.337 3.414-.587.924-.923 1.12-2.299.587-3.414l6.45-6.45z" />
				</svg>
				<span class="ml-2">Consub</span>
			</div>
			<div class="mt-5 flex flex-grow flex-col">
				<SidebarMenuItem>
					<span slot="svg-icon">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="currentColor"
							class="h-4 w-4"
							viewBox="0 0 24 24">
							<path fill="none" d="M0 0h24v24H0z" />
							<path
								d="M6.94 14.036c-.233.624-.43 1.2-.606 1.783.96-.697 2.101-1.139 3.418-1.304 2.513-.314 4.746-1.973 5.876-4.058l-1.456-1.455 1.413-1.415 1-1.001c.43-.43.915-1.224 1.428-2.368-5.593.867-9.018 4.292-11.074 9.818zM17 9.001L18 10c-1 3-4 6-8 6.5-2.669.334-4.336 2.167-5.002 5.5H3C4 16 6 2 21 2c-1 2.997-1.998 4.996-2.997 5.997L17 9.001z" />
						</svg>
					</span>
					<span slot="name">Posts</span>
					<span slot="items">
						<a href={links.get('posts')} class="menu-item">Posts</a>
						<a href={links.get('posts.categories')} class="menu-item">Categories</a>
					</span>
				</SidebarMenuItem>

				<SidebarMenuItem>
					<span slot="svg-icon">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="currentColor"
							viewBox="0 0 24 24"
							class="h-4 w-4">
							<path fill="none" d="M0 0h24v24H0z" />
							<path
								d="M10 6c0 .732-.197 1.419-.54 2.01L12 10.585l6.728-6.728a2 2 0 0 1 2.828 0l-12.11 12.11a4 4 0 1 1-1.414-1.414L10.586 12 8.032 9.446A4 4 0 1 1 10 6zM8 6a2 2 0 1 0-4 0 2 2 0 0 0 4 0zm13.556 14.142a2 2 0 0 1-2.828 0l-5.317-5.316 1.415-1.415 6.73 6.731zM16 11h2v2h-2v-2zm4 0h2v2h-2v-2zM6 11h2v2H6v-2zm-4 0h2v2H2v-2zm4 9a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
						</svg>
					</span>
					<span slot="name">Clipping</span>
					<span slot="items">
						<a href={links.get('clipping')} class="menu-item">Clippings</a>

						<a href={links.get('clipping.categories')} class="menu-item">Categories</a>
					</span>
				</SidebarMenuItem>
			</div>
		</div>
	</div>
	<div class="flex flex-1 flex-col md:pl-64">
		<main>
			<div class="sticky top-0 z-10 pl-1 pt-1 sm:pl-3 sm:pt-3 md:hidden">
				<button
					type="button"
					on:click={() => {
						sidebarOpen = true;
					}}
					class="-ml-0.5 -mt-0.5 inline-flex h-12 w-12 items-center justify-center rounded-md text-gray-500 hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
					<span class="sr-only">Open sidebar</span>
					<!-- Heroicon name: outline/bars-3 -->
					<svg
						class="h-6 w-6"
						xmlns="http://www.w3.org/2000/svg"
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
				</button>
			</div>
			<div class="mx-auto">
				<slot />
			</div>
		</main>
	</div>
</div>
<Notifier />
