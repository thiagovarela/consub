<div class="px-4 py-5 sm:px-6">
	<div class="sm:flex sm:items-center justify-between">
		<div class="flex">
			<slot name="header-left" />
		</div>
		<div class="flex">
			<slot name="header-middle" />
		</div>
		<div class="flex justify-end sm:mt-0 sm:ml-16 sm:flex-none">
			<slot name="header-actions" />
		</div>
	</div>
	<div class="mt-8 flex flex-col">
		<slot name="main" />
	</div>
</div>
