<script lang="ts">
	export let name: string;
	export let checked: boolean;
</script>

<style lang="postcss">
	.bg-enabled {
		@apply bg-emerald-600;
	}
	.bg-disabled {
		@apply bg-gray-200;
	}
	.translate-enabled {
		@apply translate-x-5;
	}
	.translate-disabled {
		@apply translate-x-0;
	}
</style>

<input type="checkbox" {name} class="hidden" bind:checked />
<button
	type="button"
	on:click={() => {
		checked = !checked;
	}}
	class="group relative inline-flex h-5 w-10 flex-shrink-0 cursor-pointer items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2"
	role="switch"
	aria-checked="false">
	<span class="sr-only">{name}</span>
	<span
		aria-hidden="true"
		class="pointer-events-none absolute h-full w-full rounded-md bg-white" />
	<!-- Enabled: "bg-emerald-600", Not Enabled: "bg-gray-200" -->
	<span
		aria-hidden="true"
		class:bg-enabled={checked}
		class:bg-disabled={!checked}
		class="pointer-events-none absolute mx-auto h-4 w-9 rounded-full transition-colors duration-200 ease-in-out" />
	<!-- Enabled: "translate-x-5", Not Enabled: "translate-x-0" -->
	<span
		aria-hidden="true"
		class:translate-enabled={checked}
		class:translate-disabled={!checked}
		class="pointer-events-none absolute left-0 inline-block h-5 w-5 transform rounded-full border border-gray-200 bg-white shadow ring-0 transition-transform duration-200 ease-in-out" />
</button>
