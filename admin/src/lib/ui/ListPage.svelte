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
				class="border border-gray-300 bg-white py-2 px-4 text-sm font-light text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none">
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
