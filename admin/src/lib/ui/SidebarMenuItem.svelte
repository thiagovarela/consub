<script>
	let open = false;
</script>

<style lang="postcss">
	.expanded {
		@apply text-gray-400 rotate-90;
	}
	.collapsed {
		@apply text-gray-300;
	}
</style>

<div class="space-y-1">
	<!-- Current: "bg-gray-100 text-gray-900", Default: "bg-white text-gray-600 hover:bg-gray-50 hover:text-gray-900" -->
	<button
		type="button"
		on:click={() => {
			open = !open;
		}}
		class="bg-white text-gray-600 hover:bg-gray-50 hover:text-gray-900 group w-full flex items-center pl-2 pr-1 py-2 text-left text-sm font-medium rounded-md focus:outline-none"
		aria-controls="side-menu"
		aria-expanded="false">
		<slot name="svg-icon" class="w-5 flex-shrink-0 text-gray-300" />

		<span class="flex-1 ml-2"><slot name="name" /></span>
		<!-- Expanded: "text-gray-400 rotate-90", Collapsed: "text-gray-300" -->
		<svg
			class:expanded={open}
			class:collapsed={!open}
			class="ml-3 h-5 w-5 flex-shrink-0 transform transition-colors duration-150 ease-in-out group-hover:text-gray-400"
			viewBox="0 0 20 20"
			aria-hidden="true">
			<path d="M6 6L14 10L6 14V6Z" fill="currentColor" />
		</svg>
	</button>
	<!-- Expandable link section, show/hide based on state. -->
	<div class="space-y-1" id="side-menu" class:hidden={!open}>
		<slot name="items" />
	</div>
</div>
