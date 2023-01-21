<script lang="ts">
	import { page } from '$app/stores';
	let sidebarOpen = false	
	
	let links = new Map()
	links.set('posts', `/admin/${$page.params.subdomain}/posts`)
	links.set('dashboard', `/admin/${$page.params.subdomain}/dashboard`)
	links.set('posts.categories', `/admin/${$page.params.subdomain}/posts/categories`)
	links.set('posts.drafts', `/admin/${$page.params.subdomain}/posts/drafts`)
	links.set('clipping', `/admin/${$page.params.subdomain}/clipping`)
	links.set('pages', `/admin/${$page.params.subdomain}/pages`)
	links.set('analytics', `/admin/${$page.params.subdomain}/pages`)
	

</script>

<!--
  This example requires updating your template:

  ```
  <html class="h-full bg-gray-100">
  <body class="h-full">
  ```
-->
<div>
	<!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
	<div class="relative z-40 hidden" role="dialog" aria-modal="true">
		<!--
        Off-canvas menu backdrop, show/hide based on off-canvas menu state.
  
        Entering: "transition-opacity ease-linear duration-300"
          From: "opacity-0"
          To: "opacity-100"
        Leaving: "transition-opacity ease-linear duration-300"
          From: "opacity-100"
          To: "opacity-0"
      -->
		<div class="fixed inset-0 bg-gray-600 bg-opacity-75 opacity-0" />

		<div class="fixed inset-0 z-40 flex">
			<!--
          Off-canvas menu, show/hide based on off-canvas menu state.
  
          Entering: "transition ease-in-out duration-300 transform"
            From: "-translate-x-full"
            To: "translate-x-0"
          Leaving: "transition ease-in-out duration-300 transform"
            From: "translate-x-0"
            To: "-translate-x-full"
        -->
			<div class="relative flex w-full max-w-xs flex-1 flex-col bg-gray-800 opacity-0">
				<!--
            Close button, show/hide based on off-canvas menu state.
  
            Entering: "ease-in-out duration-300"
              From: "opacity-0"
              To: "opacity-100"
            Leaving: "ease-in-out duration-300"
              From: "opacity-100"
              To: "opacity-0"
          -->
				<div class="absolute top-0 right-0 -mr-12 pt-2">
					<button
						on:click={() => (sidebarOpen = false)}
						type="button"
						class="ml-1 flex h-10 w-10 items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
					>
						<span class="sr-only">Close sidebar</span>
						<!-- Heroicon name: outline/x-mark -->
						<svg
							class="h-6 w-6 text-white"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					</button>
				</div>

				<div class="h-0 flex-1 overflow-y-auto pt-5 pb-4">
					<div class="flex flex-shrink-0 items-center px-4">
						<img
							class="h-8 w-auto"
							src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
							alt="Your Company"
						/>
					</div>
					<nav class="mt-5 space-y-1 px-2">
						<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
						<a
							href="#"
							class="bg-gray-900 text-white group flex items-center px-2 py-2 text-base font-medium rounded-md"
						>
							<!--
                  Heroicon name: outline/home
  
                  Current: "text-gray-300", Default: "text-gray-400 group-hover:text-gray-300"
                -->
							<svg
								class="text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
								/>
							</svg>
							Dashboard
						</a>

						<a
							href="{links.get('posts')}"
							class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-base font-medium rounded-md"
						>
							<!-- Heroicon name: outline/users -->
							<svg
								class="text-gray-400 group-hover:text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
								/>
							</svg>
							Posts
						</a>

						<a
							href="{links.get('posts.categories')}"
							class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-6 py-2 text-base font-medium rounded-md"
						>
							<!-- Heroicon name: outline/users -->
							<svg
								class="text-gray-400 group-hover:text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
								/>
							</svg>
							Categories
						</a>

						<a
							href="{links.get('leads')}"
							class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-base font-medium rounded-md"
						>
							<!-- Heroicon name: outline/folder -->
							<svg
								class="text-gray-400 group-hover:text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
								/>
							</svg>
							Leads
						</a>

						<a
							href="{links.get('newsletter')}"
							class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-base font-medium rounded-md"
						>
							<!-- Heroicon name: outline/calendar -->
							<svg
								class="text-gray-400 group-hover:text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
								/>
							</svg>
							Newsletter
						</a>

						<a
							href="{links.get('media')}"
							class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-base font-medium rounded-md"
						>
							<!-- Heroicon name: outline/inbox -->
							<svg
								class="text-gray-400 group-hover:text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 00-2.15-1.588H6.911a2.25 2.25 0 00-2.15 1.588L2.35 13.177a2.25 2.25 0 00-.1.661z"
								/>
							</svg>
							Media Library
						</a>

						<a
							href="{links.get('analytics')}"
							class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-base font-medium rounded-md"
						>
							<!-- Heroicon name: outline/chart-bar -->
							<svg
								class="text-gray-400 group-hover:text-gray-300 mr-4 flex-shrink-0 h-6 w-6"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								aria-hidden="true"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
								/>
							</svg>
							Analytics
						</a>
					</nav>
				</div>
				<div class="flex flex-shrink-0 bg-gray-700 p-4">
					<a href="{links.get('profile')}" class="group block flex-shrink-0">
						<div class="flex items-center">
							<div>
								<img
									class="inline-block h-10 w-10 rounded-full"
									src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
									alt=""
								/>
							</div>
							<div class="ml-3">
								<p class="text-base font-medium text-white">Tom Cook</p>
								<p
									class="text-sm font-medium text-gray-400 group-hover:text-gray-300"
								>
									View profile
								</p>
							</div>
						</div>
					</a>
				</div>
			</div>

			<div class="w-14 flex-shrink-0">
				<!-- Force sidebar to shrink to fit close icon -->
			</div>
		</div>
	</div>

	<!-- Static sidebar for desktop -->
	<div class="hidden md:fixed md:inset-y-0 md:flex md:w-64 md:flex-col">
		<!-- Sidebar component, swap this element with another sidebar if you like -->
		<div class="flex min-h-0 flex-1 flex-col bg-gray-800">
			<div class="flex flex-1 flex-col overflow-y-auto pt-5 pb-4">
				<div class="flex flex-shrink-0 items-center px-4 hidden">
					<img
						class="h-8 w-auto"
						src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
						alt="Your Company"
					/>
				</div>
				<nav class="mt-5 flex-1 space-y-1 px-2">
					<!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
					<a
						href="{links.get('dashboard')}"
						class="bg-gray-900 text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!--
                Heroicon name: outline/home
  
                Current: "text-gray-300", Default: "text-gray-400 group-hover:text-gray-300"
              -->
						<svg
							class="text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
							/>
						</svg>
						Dashboard
					</a>

					<a
						href="{links.get('posts')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!-- Heroicon name: outline/users -->
						<svg
							class="text-gray-400 group-hover:text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10"
							/>
						</svg>
						Posts
					</a>

					<a
						href="{links.get('drafts')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-6 py-2 text-sm font-medium rounded-md"
					>
						Drafts
					</a>

					<a
						href="{links.get('categories')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-6 py-2 text-sm font-medium rounded-md"
					>
						Categories
					</a>

					<a
						href="{links.get('pages')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!-- Heroicon name: outline/users -->
						<svg
							class="text-gray-400 group-hover:text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
							/>
						</svg>
						Pages
					</a>

					<a
						href="{links.get('leads')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!-- Heroicon name: outline/folder -->
						<svg
							class="text-gray-400 group-hover:text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
							/>
						</svg>
						Leads
					</a>

					<a
						href="{links.get('clipping')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!-- Heroicon name: outline/calendar -->
						<svg
							class="text-gray-400 group-hover:text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
							/>
						</svg>
						Clipping
					</a>

					<a
						href="{links.get('media')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!-- Heroicon name: outline/inbox -->
						<svg
							class="text-gray-400 group-hover:text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 00-2.15-1.588H6.911a2.25 2.25 0 00-2.15 1.588L2.35 13.177a2.25 2.25 0 00-.1.661z"
							/>
						</svg>
						Media Library
					</a>

					<a
						href="{links.get('analytics')}"
						class="text-gray-300 hover:bg-gray-700 hover:text-white group flex items-center px-2 py-2 text-sm font-medium rounded-md"
					>
						<!-- Heroicon name: outline/chart-bar -->
						<svg
							class="text-gray-400 group-hover:text-gray-300 mr-3 flex-shrink-0 h-6 w-6"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							aria-hidden="true"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
							/>
						</svg>
						Analytics
					</a>
				</nav>
			</div>
			<div class="flex flex-shrink-0 bg-gray-700 p-4">
				<a href="{links.get('profile')}" class="group block w-full flex-shrink-0">
					<div class="flex items-center">
						<div>
							<img
								class="inline-block h-9 w-9 rounded-full"
								src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
								alt=""
							/>
						</div>
						<div class="ml-3">
							<p class="text-sm font-medium text-white">Tom Cook</p>
							<p class="text-xs font-medium text-gray-300 group-hover:text-gray-200">
								View profile
							</p>
						</div>
					</div>
				</a>
			</div>
		</div>
	</div>
	<div class="flex flex-1 flex-col md:pl-64">
		<div class="sticky top-0 z-10 bg-gray-100 pl-1 pt-1 sm:pl-3 sm:pt-3 md:hidden">
			<button
				type="button"
				class="-ml-0.5 -mt-0.5 inline-flex h-12 w-12 items-center justify-center rounded-md text-gray-500 hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
			>
				<span class="sr-only">Open sidebar</span>
				<!-- Heroicon name: outline/bars-3 -->
				<svg
					class="h-6 w-6"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					aria-hidden="true"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
					/>
				</svg>
			</button>
		</div>
		<main class="flex-1">
			<slot />
		</main>
	</div>
</div>
