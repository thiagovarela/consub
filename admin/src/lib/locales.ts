const DEFAULT_LOCALE = 'en-US';
export const supportedlocales = ['pt-PT', 'pt-BR', 'en-US', 'es-ES', 'fr-FR'];

interface Locale {
	label: string;
	value: string;
}

export const locales = (from: readonly string[]): Locale[] => {
	let display = new Intl.DisplayNames(from, { type: 'language' });
	return supportedlocales.map((locale) => {
		return {
			label: display.of(locale) || locale,
			value: locale
		};
	});
};

export const parse = (acceptHeader: string | null | undefined) => {
	if (!acceptHeader) {
		return [];
	}
	const langs = acceptHeader.split(/,|;q=/g);

	return langs.reverse().reduce((e, val: string) => {
		const [code, region] = val.split('-');

		if (region) {
			e.push({ code, region });
		}
		return e;
	}, []);
};

export const getHeaderLanguage = (acceptHeader: string | null | undefined): string => {
	let langs = parse(acceptHeader);
	if (langs.length === 0) {
		return DEFAULT_LOCALE;
	}
	let lang = langs[0];
	return `${lang.code}-${lang.region}`;
};
