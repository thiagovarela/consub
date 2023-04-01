<svelte:options accessors={true} />

<script lang="ts">
	export let show: boolean = false;

	let dialog: HTMLDialogElement;

	$: if (dialog && show) dialog.showModal();
	$: if (dialog && !show) dialog.close();
</script>

<style>
	dialog {
		max-width: 64em;
		border-radius: 0.2em;
		border: none;
		padding: 0;
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}
	dialog > div {
		padding: 1em;
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
</style>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog bind:this={dialog} on:close={() => (show = false)} on:click|self={() => dialog.close()}>
	<div on:click|stopPropagation>
		<slot name="header" />
		<slot />
		<!-- svelte-ignore a11y-autofocus -->
		<button
			autofocus
			on:click={() => dialog.close()}
			type="button"
			class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:col-start-1 sm:mt-0">
			Cancel
		</button>
	</div>
</dialog>
