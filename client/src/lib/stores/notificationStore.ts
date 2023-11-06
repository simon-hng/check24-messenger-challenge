import { get, writable } from 'svelte/store';
import { userStore } from './userStore';
import { browser } from '$app/environment';
import type { WSNotification } from '$lib/types';

const createNotificationStore = () => {
	const user = get(userStore);
	const { subscribe, set } = writable<WSNotification>();

	let socket: WebSocket;

	if (browser) {
		socket = new WebSocket('ws://localhost:8080/notifications/ws');

		socket.onopen = () => {
			socket.send(JSON.stringify({ type: 'Auth', id: user?.id }));
		};
		socket.onmessage = (event) => {
			const notification = JSON.parse(event.data);
			set(notification);
		};
	}

	return {
		subscribe,
		close: () => socket.close()
	};
};

export const notificationStore = createNotificationStore();
