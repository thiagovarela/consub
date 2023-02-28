import { writable } from 'svelte/store';

export interface Notification {
	message: string;
	type: 'info' | 'success' | 'warning' | 'error';
	more?: string;
}

export const notification = writable<Notification | undefined>();

const notify = (message: string, type: 'info' | 'success' | 'warning' | 'error', more?: string) => {
	notification.set({ message, type, more });
	setTimeout(() => notification.set(undefined), 6000);
};
export const notifier = {
	info: (message: string, more?: string) => notify(message, 'info', more),
	success: (message: string, more?: string) => notify(message, 'success', more),
	warning: (message: string, more?: string) => notify(message, 'warning', more),
	error: (message: string, more?: string) => notify(message, 'error', more)
};
