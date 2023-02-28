<script lang="ts">
	import { goto } from '$app/navigation';

	export let name: string;
	export let namePlural: string;
	export let editUrl: string;
</script>

<div class="px-4 py-5 sm:px-6">
	<div class="sm:flex sm:items-center">
		<div class="sm:flex-auto">
			<h1 class="text-xl font-semibold text-gray-900">
				{`${namePlural[0].toUpperCase()}${namePlural.slice(1).toLowerCase()}`}
			</h1>
		</div>
		<div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
			<button
				type="button"
				on:click={() => goto(editUrl)}
				class="inline-flex items-center justify-center  border border-transparent bg-emerald-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 sm:w-auto">
				Add {name}
			</button>
		</div>
	</div>
	<div class="mt-8 flex flex-col">
		<div class="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
				<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
					<slot name="table" />
				</div>
			</div>
		</div>
	</div>
</div>
