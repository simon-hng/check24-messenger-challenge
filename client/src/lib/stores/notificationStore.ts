import { get, writable } from 'svelte/store';
import { userStore } from './userStore';
import { browser } from '$app/environment';

const createNotificationStore = () => {
	const user = get(userStore);
	const { subscribe, update } = writable<any[]>([]);

	let socket: WebSocket;

	if (browser) {
		socket = new WebSocket('ws://localhost:8080/notifications/ws');

		socket.onopen = () => {
			socket.send(JSON.stringify({ type: 'Auth', id: user?.name }));
		};
		socket.onmessage = (event) => {
			update((notifications) => [...notifications, event.data]);
		};
	}

	return {
		subscribe,
		close: () => socket.close()
	};
};

export const notificationStore = createNotificationStore();
