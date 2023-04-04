<svelte:options accessors />

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor, isActive } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';
	import CharacterCount from '@tiptap/extension-character-count';
	import Link from '@tiptap/extension-link';
	import Placeholder from '@tiptap/extension-placeholder';
	import { TipTapImageSet } from '$lib/tiptap/imageset';

	import MediaUpload from '$lib/ui/MediaUpload.svelte';
	import type { ImageSet } from './api';
	import { buildSrc, buildSrcset } from './images';
	import BubbleMenu from '$lib/tiptap/BubbleMenu.svelte';
	import Modal from '$lib/ui/Modal.svelte';

	let element: Element;
	let editor: Editor;

	let imageUploadModal: boolean = false;

	export let contentJsonString: string | undefined;
	export let contentHtml: string | undefined;
	export let contentText: string | undefined;

	onMount(() => {
		editor = new Editor({
			element: element,
			extensions: [
				StarterKit,
				CharacterCount.configure({}),
				Link.configure({
					openOnClick: false,
				}).extend({
					// @ts-ignore
					addKeyboardShortcuts: () => {
						return {
							'Mod-Shift-k': (event) => {
								const editor = event.editor;
								const { state } = editor.view;
								const { from, to } = state.selection;
								const node = state.doc.nodeAt(from);
								if (node && node.type.name === 'link') {
									editor.chain().focus().unsetLink().run();
								} else {
									const previousUrl = editor.getAttributes('link').href;
									const url = window.prompt('URL', previousUrl);
									// cancelled
									if (url === null) {
										return;
									}
									// empty
									if (url === '') {
										editor.chain().focus().extendMarkRange('link').unsetLink().run();
										return;
									}
									// update link
									editor.chain().focus().extendMarkRange('link').setLink({ href: url }).run();
								}
							},
						};
					},
				}),
				TipTapImageSet.configure({}),
				Placeholder.configure({
					placeholder: 'Start typing...'
				})
			],
			editorProps: {
				attributes: {
					class: 'prose prose-sm sm:prose px-2 lg:prose-lg focus:outline-none mt-8 mx-auto items-center'
				}
			},
			content: contentJsonString ? JSON.parse(contentJsonString) : null,
			onTransaction: () => {
				// force re-render so `editor.isActive` works as expected
				editor = editor;
			}
		});
		editor.on('update', () => {
			contentJsonString = JSON.stringify(editor.getJSON());
			contentHtml = editor.getHTML();
			contentText = editor.getText();
		});

		editor.on('selectionUpdate', ({ editor }) => {
			if (editor.isActive('image')) {
				const { state } = editor.view;
				const { from, to } = state.selection;
				const node = state.doc.nodeAt(from);
				if (node && node.type.name === 'image') {
					const { src, srcset, alt, title } = node.attrs;					
				}
			}
		});
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});

	const onMediaSelect = (imageSet: ImageSet[]) => {
		let srcset = imageSet.map(buildSrcset).join(', ');
		let src = buildSrc(imageSet[0]);
		editor.chain().focus().setImageSet({ srcset, src }).run();
	};
</script>

<style lang="postcss">
	.editor-toolbar {
		@apply flex flex-wrap items-center justify-between md:px-2 md:py-1;
	}
	.editor-toolbar button {
		@apply inline-flex items-center rounded border border-transparent px-2.5 py-1.5 text-xs
         font-medium text-white shadow-sm hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2
		 disabled:opacity-50 disabled:cursor-not-allowed;
	}
	.editor-toolbar button.active {
		@apply bg-slate-700;
	}
	.editor-toolbar svg {
		@apply w-4 h-4 md:w-5 md:h-5;
	}
	:global(.ProseMirror a) {
		@apply text-slate-500 hover:text-slate-700;
	}
	:global(img.ProseMirror-selectednode) {
		@apply ring-2 ring-slate-500;
	}
	:global(.ProseMirror p.is-editor-empty:first-child::before) {
		color: #adb5bd;
		content: attr(data-placeholder);
		float: left;
		height: 0;
		pointer-events: none;
	}
	.editor-container {
		@apply w-full bg-white h-full md:pb-8;
	}
	.bubble-menu {
		display: flex;
		background-color: #0d0d0d;
		padding: 0.2rem;
		border-radius: 0.5rem;
		transition: visibility 0.1s ease, opacity 0.1s ease;
		opacity: 0;
	}
	.bubble-menu.is-active {
		opacity: 1;
	}
</style>

<div class="flex w-full border flex-col rounded-md">
	{#if editor}
		<div>
			<div class="editor-toolbar">
				<button
					type="button"
					title="Toggle bold"
					on:click={() => editor.chain().focus().toggleBold().run()}
					disabled={!editor.can().chain().focus().toggleBold().run()}
					class:active={editor.isActive('bold')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M8 11h4.5a2.5 2.5 0 1 0 0-5H8v5zm10 4.5a4.5 4.5 0 0 1-4.5 4.5H6V4h6.5a4.5 4.5 0 0 1 3.256 7.606A4.498 4.498 0 0 1 18 15.5zM8 13v5h5.5a2.5 2.5 0 1 0 0-5H8z" />
					</svg>
				</button>
				<button
					type="button"
					on:click={() => editor.chain().focus().toggleItalic().run()}
					disabled={!editor.can().chain().focus().toggleItalic().run()}
					class:active={editor.isActive('italic')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path d="M15 20H7v-2h2.927l2.116-12H9V4h8v2h-2.927l-2.116 12H15z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle strikethrough"
					on:click={() => editor.chain().focus().toggleStrike().run()}
					disabled={!editor.can().chain().focus().toggleStrike().run()}
					class:active={editor.isActive('strike')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M17.154 14c.23.516.346 1.09.346 1.72 0 1.342-.524 2.392-1.571 3.147C14.88 19.622 13.433 20 11.586 20c-1.64 0-3.263-.381-4.87-1.144V16.6c1.52.877 3.075 1.316 4.666 1.316 2.551 0 3.83-.732 3.839-2.197a2.21 2.21 0 0 0-.648-1.603l-.12-.117H3v-2h18v2h-3.846zm-4.078-3H7.629a4.086 4.086 0 0 1-.481-.522C6.716 9.92 6.5 9.246 6.5 8.452c0-1.236.466-2.287 1.397-3.153C8.83 4.433 10.271 4 12.222 4c1.471 0 2.879.328 4.222.984v2.152c-1.2-.687-2.515-1.03-3.946-1.03-2.48 0-3.719.782-3.719 2.346 0 .42.218.786.654 1.099.436.313.974.562 1.613.75.62.18 1.297.414 2.03.699z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle heading 1"
					on:click={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
					disabled={!editor.can().chain().focus().toggleHeading({ level: 1 }).run()}
					class:active={editor.isActive('heading', { level: 1 })}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M13 20h-2v-7H4v7H2V4h2v7h7V4h2v16zm8-12v12h-2v-9.796l-2 .536V8.67L19.5 8H21z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle heading 2"
					on:click={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
					disabled={!editor.can().chain().focus().toggleHeading({ level: 2 }).run()}
					class:active={editor.isActive('heading', { level: 2 })}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M4 4v7h7V4h2v16h-2v-7H4v7H2V4h2zm14.5 4c2.071 0 3.75 1.679 3.75 3.75 0 .857-.288 1.648-.772 2.28l-.148.18L18.034 18H22v2h-7v-1.556l4.82-5.546c.268-.307.43-.709.43-1.148 0-.966-.784-1.75-1.75-1.75-.918 0-1.671.707-1.744 1.606l-.006.144h-2C14.75 9.679 16.429 8 18.5 8z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle heading 3"
					on:click={() => editor.chain().focus().toggleHeading({ level: 3 }).run()}
					disabled={!editor.can().chain().focus().toggleHeading({ level: 3 }).run()}
					class:active={editor.isActive('heading', { level: 3 })}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M22 8l-.002 2-2.505 2.883c1.59.435 2.757 1.89 2.757 3.617 0 2.071-1.679 3.75-3.75 3.75-1.826 0-3.347-1.305-3.682-3.033l1.964-.382c.156.806.866 1.415 1.718 1.415.966 0 1.75-.784 1.75-1.75s-.784-1.75-1.75-1.75c-.286 0-.556.069-.794.19l-1.307-1.547L19.35 10H15V8h7zM4 4v7h7V4h2v16h-2v-7H4v7H2V4h2z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle heading 4"
					on:click={() => editor.chain().focus().toggleHeading({ level: 4 }).run()}
					disabled={!editor.can().chain().focus().toggleHeading({ level: 4 }).run()}
					class:active={editor.isActive('heading', { level: 4 })}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M13 20h-2v-7H4v7H2V4h2v7h7V4h2v16zm9-12v8h1.5v2H22v2h-2v-2h-5.5v-1.34l5-8.66H22zm-2 3.133L17.19 16H20v-4.867z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle heading 5"
					on:click={() => editor.chain().focus().toggleHeading({ level: 5 }).run()}
					disabled={!editor.can().chain().focus().toggleHeading({ level: 5 }).run()}
					class:active={editor.isActive('heading', { level: 5 })}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M22 8v2h-4.323l-.464 2.636c.33-.089.678-.136 1.037-.136 2.21 0 4 1.79 4 4s-1.79 4-4 4c-1.827 0-3.367-1.224-3.846-2.897l1.923-.551c.24.836 1.01 1.448 1.923 1.448 1.105 0 2-.895 2-2s-.895-2-2-2c-.63 0-1.193.292-1.56.748l-1.81-.904L16 8h6zM4 4v7h7V4h2v16h-2v-7H4v7H2V4h2z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle heading 6"
					on:click={() => editor.chain().focus().toggleHeading({ level: 6 }).run()}
					disabled={!editor.can().chain().focus().toggleHeading({ level: 6 }).run()}
					class:active={editor.isActive('heading', { level: 6 })}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0H24V24H0z" />
						<path
							d="M21.097 8l-2.598 4.5c2.21 0 4.001 1.79 4.001 4s-1.79 4-4 4-4-1.79-4-4c0-.736.199-1.426.546-2.019L18.788 8h2.309zM4 4v7h7V4h2v16h-2v-7H4v7H2V4h2zm14.5 10.5c-1.105 0-2 .895-2 2s.895 2 2 2 2-.895 2-2-.895-2-2-2z" />
					</svg>
				</button>
				<button
					title="Toggle link"
					on:click={() => {
						const previousUrl = editor.getAttributes('link').href;
						const url = window.prompt('URL', previousUrl);
						// cancelled
						if (url === null) {
							return;
						}
						// empty
						if (url === '') {
							editor.chain().focus().extendMarkRange('link').unsetLink().run();
							return;
						}
						// update link
						editor.chain().focus().extendMarkRange('link').setLink({ href: url }).run();
					}}
					type="button">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						{#if !editor.getAttributes('link').href }
						<path
							d="M17.6572 14.8282L16.2429 13.414L17.6572 11.9998C19.2193 10.4377 19.2193 7.90506 17.6572 6.34296C16.0951 4.78086 13.5624 4.78086 12.0003 6.34296L10.5861 7.75717L9.17188 6.34296L10.5861 4.92875C12.9292 2.5856 16.7282 2.5856 19.0714 4.92875C21.4145 7.27189 21.4145 11.0709 19.0714 13.414L17.6572 14.8282ZM14.8287 17.6567L13.4145 19.0709C11.0714 21.414 7.27238 21.414 4.92923 19.0709C2.58609 16.7277 2.58609 12.9287 4.92923 10.5856L6.34345 9.17139L7.75766 10.5856L6.34345 11.9998C4.78135 13.5619 4.78135 16.0946 6.34345 17.6567C7.90555 19.2188 10.4382 19.2188 12.0003 17.6567L13.4145 16.2425L14.8287 17.6567ZM14.8287 7.75717L16.2429 9.17139L9.17188 16.2425L7.75766 14.8282L14.8287 7.75717Z"
							fill="#000" />
						{/if}
						{#if editor.isActive('link') && editor.getAttributes('link').href }
							<path
								d="M17.657 14.8286L16.2428 13.4143L17.657 12.0001C19.2191 10.438 19.2191 7.90538 17.657 6.34328C16.0949 4.78118 13.5622 4.78118 12.0001 6.34328L10.5859 7.75749L9.17171 6.34328L10.5859 4.92907C12.9291 2.58592 16.7281 2.58592 19.0712 4.92907C21.4143 7.27221 21.4143 11.0712 19.0712 13.4143L17.657 14.8286ZM14.8286 17.657L13.4143 19.0712C11.0712 21.4143 7.27221 21.4143 4.92907 19.0712C2.58592 16.7281 2.58592 12.9291 4.92907 10.5859L6.34328 9.17171L7.75749 10.5859L6.34328 12.0001C4.78118 13.5622 4.78118 16.0949 6.34328 17.657C7.90538 19.2191 10.438 19.2191 12.0001 17.657L13.4143 16.2428L14.8286 17.657ZM14.8286 7.75749L16.2428 9.17171L9.17171 16.2428L7.75749 14.8286L14.8286 7.75749ZM5.77539 2.29303L7.70724 1.77539L8.74252 5.63909L6.81067 6.15673L5.77539 2.29303ZM15.2578 18.3612L17.1896 17.8435L18.2249 21.7072L16.293 22.2249L15.2578 18.3612ZM2.29303 5.77539L6.15673 6.81067L5.63909 8.74252L1.77539 7.70724L2.29303 5.77539ZM18.3612 15.2578L22.2249 16.293L21.7072 18.2249L17.8435 17.1896L18.3612 15.2578Z" />
						{/if}
					</svg>
				</button>			
				<button
					type="button"
					on:click={() => editor.chain().focus().toggleBulletList().run()}
					title="Toggle bullet list"
					class:active={editor.isActive('bulletList')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M8 4h13v2H8V4zM4.5 6.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm0 7a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm0 6.9a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zM8 11h13v2H8v-2zm0 7h13v2H8v-2z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle ordered list"
					on:click={() => editor.chain().focus().toggleOrderedList().run()}
					class:active={editor.isActive('orderedList')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M8 4h13v2H8V4zM5 3v3h1v1H3V6h1V4H3V3h2zM3 14v-2.5h2V11H3v-1h3v2.5H4v.5h2v1H3zm2 5.5H3v-1h2V18H3v-1h3v4H3v-1h2v-.5zM8 11h13v2H8v-2zm0 7h13v2H8v-2z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle code block"
					on:click={() => editor.chain().focus().toggleCodeBlock().run()}
					class:active={editor.isActive('codeBlock')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm1 2v14h16V5H4zm16 7l-3.536 3.536-1.414-1.415L17.172 12 15.05 9.879l1.414-1.415L20 12zM6.828 12l2.122 2.121-1.414 1.415L4 12l3.536-3.536L8.95 9.88 6.828 12zm4.416 5H9.116l3.64-10h2.128l-3.64 10z" />
					</svg>
				</button>
				<button
					type="button"
					title="Toggle blockquote"
					on:click={() => editor.chain().focus().toggleBlockquote().run()}
					class:active={editor.isActive('blockquote')}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M4.583 17.321C3.553 16.227 3 15 3 13.011c0-3.5 2.457-6.637 6.03-8.188l.893 1.378c-3.335 1.804-3.987 4.145-4.247 5.621.537-.278 1.24-.375 1.929-.311 1.804.167 3.226 1.648 3.226 3.489a3.5 3.5 0 0 1-3.5 3.5c-1.073 0-2.099-.49-2.748-1.179zm10 0C13.553 16.227 13 15 13 13.011c0-3.5 2.457-6.637 6.03-8.188l.893 1.378c-3.335 1.804-3.987 4.145-4.247 5.621.537-.278 1.24-.375 1.929-.311 1.804.167 3.226 1.648 3.226 3.489a3.5 3.5 0 0 1-3.5 3.5c-1.073 0-2.099-.49-2.748-1.179z" />
					</svg>
				</button>
				<button
					type="button"
					title="Set horizontal rule"
					on:click={() => editor.chain().focus().setHorizontalRule().run()}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path d="M2 11h2v2H2v-2zm4 0h12v2H6v-2zm14 0h2v2h-2v-2z" />
					</svg>
				</button>
				<button
					type="button"
					title="Set hard break"
					on:click={() => editor.chain().focus().setHardBreak().run()}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M17 21v-4H7v4H5v-5a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v5h-2zM7 3v4h10V3h2v5a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V3h2zM2 9l4 3-4 3V9zm20 0v6l-4-3 4-3z" />
					</svg>
				</button>
				<button
					type="button"
					title="Undo"
					on:click={() => editor.chain().focus().undo().run()}
					disabled={!editor.can().chain().focus().undo().run()}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M5.828 7l2.536 2.536L6.95 10.95 2 6l4.95-4.95 1.414 1.414L5.828 5H13a8 8 0 1 1 0 16H4v-2h9a6 6 0 1 0 0-12H5.828z" />
					</svg>
				</button>
				<button
					type="button"
					title="Redo"
					on:click={() => editor.chain().focus().redo().run()}
					disabled={!editor.can().chain().focus().redo().run()}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M18.172 7H11a6 6 0 1 0 0 12h9v2h-9a8 8 0 1 1 0-16h7.172l-2.536-2.536L17.05 1.05 22 6l-4.95 4.95-1.414-1.414L18.172 7z" />
					</svg>
				</button>
				<button
					type="button"
					title="Insert image"
					on:click={() => (imageUploadModal = true)}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						width="24"
						height="24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M5 11.1l2-2 5.5 5.5 3.5-3.5 3 3V5H5v6.1zm0 2.829V19h3.1l2.986-2.985L7 11.929l-2 2zM10.929 19H19v-2.071l-3-3L10.929 19zM4 3h16a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm11.5 7a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z" />
					</svg>
				</button>
			</div>
		</div>
	{/if}

	<div class="editor-container" bind:this={element} />
</div>
{#if editor}
	<!-- <div>
		{editor.storage.characterCount.characters()} characters
		<br />
		{editor.storage.characterCount.words()} words
	</div> -->
	<BubbleMenu
		bind:editor
		shouldShow={({ editor }) => {
			return editor.isActive('image');
		}}>
		<div>
			<div class="editor-toolbar">
				<button
					on:click={() =>
						editor
							.chain()
							.focus()
							.updateAttributes('image', {
								style: 'aspect-ratio: 16 / 9;',
								width: '50%'
							})
							.run()}
					type="button">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M8 11h4.5a2.5 2.5 0 1 0 0-5H8v5zm10 4.5a4.5 4.5 0 0 1-4.5 4.5H6V4h6.5a4.5 4.5 0 0 1 3.256 7.606A4.498 4.498 0 0 1 18 15.5zM8 13v5h5.5a2.5 2.5 0 1 0 0-5H8z" />
					</svg>
				</button>
			</div>
		</div>
	</BubbleMenu>

	<BubbleMenu
		bind:editor
		shouldShow={({ editor }) => {
			return editor.isActive('link') && editor.getAttributes('link').href;
		}}>
		<div>
			<div class="editor-toolbar">
				<button
					on:click={() => {
						const previousUrl = editor.getAttributes('link').href;
						const url = window.prompt('URL', previousUrl);
						// cancelled
						if (url === null) {
							return;
						}
						// empty
						if (url === '') {
							editor.chain().focus().extendMarkRange('link').unsetLink().run();
							return;
						}
						// update link
						editor.chain().focus().extendMarkRange('link').setLink({ href: url }).run();
					}}
					type="button">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M17.6572 14.8282L16.2429 13.414L17.6572 11.9998C19.2193 10.4377 19.2193 7.90506 17.6572 6.34296C16.0951 4.78086 13.5624 4.78086 12.0003 6.34296L10.5861 7.75717L9.17188 6.34296L10.5861 4.92875C12.9292 2.5856 16.7282 2.5856 19.0714 4.92875C21.4145 7.27189 21.4145 11.0709 19.0714 13.414L17.6572 14.8282ZM14.8287 17.6567L13.4145 19.0709C11.0714 21.414 7.27238 21.414 4.92923 19.0709C2.58609 16.7277 2.58609 12.9287 4.92923 10.5856L6.34345 9.17139L7.75766 10.5856L6.34345 11.9998C4.78135 13.5619 4.78135 16.0946 6.34345 17.6567C7.90555 19.2188 10.4382 19.2188 12.0003 17.6567L13.4145 16.2425L14.8287 17.6567ZM14.8287 7.75717L16.2429 9.17139L9.17188 16.2425L7.75766 14.8282L14.8287 7.75717Z"
							fill="#000" />
					</svg>
				</button>
				<button
					on:click={() => {
						editor.chain().focus().extendMarkRange('link').unsetLink().run();
					}}
					disabled={!editor.isActive('link')}
					type="button">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
						<path fill="none" d="M0 0h24v24H0z" />
						<path
							d="M17.657 14.8286L16.2428 13.4143L17.657 12.0001C19.2191 10.438 19.2191 7.90538 17.657 6.34328C16.0949 4.78118 13.5622 4.78118 12.0001 6.34328L10.5859 7.75749L9.17171 6.34328L10.5859 4.92907C12.9291 2.58592 16.7281 2.58592 19.0712 4.92907C21.4143 7.27221 21.4143 11.0712 19.0712 13.4143L17.657 14.8286ZM14.8286 17.657L13.4143 19.0712C11.0712 21.4143 7.27221 21.4143 4.92907 19.0712C2.58592 16.7281 2.58592 12.9291 4.92907 10.5859L6.34328 9.17171L7.75749 10.5859L6.34328 12.0001C4.78118 13.5622 4.78118 16.0949 6.34328 17.657C7.90538 19.2191 10.438 19.2191 12.0001 17.657L13.4143 16.2428L14.8286 17.657ZM14.8286 7.75749L16.2428 9.17171L9.17171 16.2428L7.75749 14.8286L14.8286 7.75749ZM5.77539 2.29303L7.70724 1.77539L8.74252 5.63909L6.81067 6.15673L5.77539 2.29303ZM15.2578 18.3612L17.1896 17.8435L18.2249 21.7072L16.293 22.2249L15.2578 18.3612ZM2.29303 5.77539L6.15673 6.81067L5.63909 8.74252L1.77539 7.70724L2.29303 5.77539ZM18.3612 15.2578L22.2249 16.293L21.7072 18.2249L17.8435 17.1896L18.3612 15.2578Z" />
					</svg>
				</button>
			</div>
		</div>
	</BubbleMenu>
{/if}
<Modal bind:show={imageUploadModal}>
	<MediaUpload {onMediaSelect} showTabs={true} />
</Modal>
