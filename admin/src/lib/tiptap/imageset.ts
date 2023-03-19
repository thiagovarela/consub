import { mergeAttributes, Node, nodeInputRule } from '@tiptap/core';

export interface ImageSetOptions {
	inline: boolean;
	HTMLAttributes: Record<string, any>;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		imageset: {
			/**
			 * Add an image
			 */
			setImageSet: (options: { srcset: string; alt?: string; title?: string }) => ReturnType;
		};
	}
}

export const inputRegex = /(?:^|\s)(!\[(.+|:?)]\((\S+)(?:(?:\s+)["'](\S+)["'])?\))$/;

export const ImageSet = Node.create<ImageSetOptions>({
	name: 'imageset',

	addOptions() {
		return {
			inline: false,
			HTMLAttributes: {
				sizes: 'sizes="(max-width: 640px) 640w, (max-width: 1920px) 1920w, (max-width: 2400px) 2400w"',
				style: 'aspect-ratio: auto;'
			}
		};
	},

	inline() {
		return this.options.inline;
	},

	group() {
		return this.options.inline ? 'inline' : 'block';
	},

	draggable: true,

	addAttributes() {
		return {
			srcset: {
				default: null
			},
			alt: {
				default: null
			},
			title: {
				default: null
			}
		};
	},

	parseHTML() {
		return [
			{
				tag: 'img[src]:not([src^="data:"])'
			}
		];
	},

	renderHTML({ HTMLAttributes }) {
		return ['img', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes)];
	},

	addCommands() {
		return {
			setImageSet:
				(options) =>
				({ commands }) => {
					console.log(options);
					return commands.insertContent({
						type: this.name,
						attrs: options
					});
				}
		};
	},

	addInputRules() {
		return [
			nodeInputRule({
				find: inputRegex,
				type: this.type,
				getAttributes: (match) => {
					const [, , alt, srcset, title] = match;
					return { srcset, alt, title };
				}
			})
		];
	}
});
