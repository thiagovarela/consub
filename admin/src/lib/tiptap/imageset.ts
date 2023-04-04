import type { srcset } from '$lib/images';
import { mergeAttributes, Node, nodeInputRule } from '@tiptap/core';

export interface ImageOptions {
	inline: boolean;
	allowBase64: boolean;
	HTMLAttributes: Record<string, any>;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		image: {
			/**
			 * Add an image
			 */
			setImageSet: (options: {
				src: string;
				srcset: string;
				alt?: string;
				title?: string;
			}) => ReturnType;
		};
	}
}

export const inputRegex = /(?:^|\s)(!\[(.+|:?)]\((\S+)(?:(?:\s+)["'](\S+)["'])?\))$/;

export const TipTapImageSet = Node.create<ImageOptions>({
	name: 'image',

	addOptions() {
		return {
			inline: false,
			allowBase64: false,
			HTMLAttributes: { loading: 'lazy' }
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
			src: {
				default: null
			},
			srcset: {
				default: null
			},
			alt: {
				default: null
			},
			title: {
				default: null
			},
			style: {
				default: null
			},
			width: {
				default: null
			}
		};
	},

	parseHTML() {
		return [
			{
				tag: this.options.allowBase64 ? 'img[src]' : 'img[src]:not([src^="data:"])'
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
					const [, , alt, src, title] = match;

					return { src, alt, title };
				}
			})
		];
	}
});
