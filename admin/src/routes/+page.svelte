<div class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
	<div class="sm:mx-auto sm:w-full sm:max-w-md">
		<h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
			Sign in to your account
		</h2>
		<p class="mt-2 text-center text-sm text-gray-600">
			Or
			<a href="/create-account" class="font-medium text-slate-600 hover:text-slate-500">
				create one
			</a>
		</p>
	</div>

	<div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
		<div class="bg-white py-8 px-4 lg:shadow sm:rounded-lg sm:px-10">
			<form class="space-y-6" action="/check-account" method="POST">
				<div>
					<label for="subdomain" class="block text-sm font-medium text-gray-700">
						Account identifier
					</label>
					<div class="mt-1 sm:col-span-2 sm:mt-0">
						<input
							id="subdomain"
							name="subdomain"
							type="text"
							autocomplete="subdomain"
							required
							class="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-slate-300 focus:outline-none focus:ring-slate-300 sm:text-sm" />
					</div>
				</div>

				<div>
					<button
						type="submit"
						class="flex w-full justify-center rounded-md border border-transparent bg-slate-500 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2">
						Next
					</button>
				</div>
			</form>
		</div>
	</div>
</div>
