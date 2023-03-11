<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="text-center">
	<h3 class="mt-2 text-sm font-semibold text-gray-900">{$page.status}</h3>
	<p class="mt-1 text-sm text-gray-500">{$page.error?.message}</p>
</div>
