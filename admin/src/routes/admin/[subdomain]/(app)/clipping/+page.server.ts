import { ClippingsService } from '$lib/api';

export async function load() {
	let items = await ClippingsService.listItems({});
	let categoriesIds: string[] = items.map((item) => item.category_id);
	
	let promises = categoriesIds.map((id) => { return ClippingsService.getCategoryById({categoryId: id}) });	
	let categories = await Promise.all(promises);

	return {
		items,
		categories
	};
};
