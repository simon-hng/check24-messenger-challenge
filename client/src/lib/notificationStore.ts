import { get, writable } from 'svelte/store';
import { userStore } from './userStore';

function createNotificationStore() {
	const user = get(userStore);

	if (!user) {
		return;
	}

	const { subscribe, update } = writable<any[]>([]);

	const socket = new WebSocket('ws://localhost:8080/message/ws');

	socket.onopen = () => {
		socket.send(JSON.stringify({ type: 'Auth', id: user.name }));
	};
	socket.onmessage = (event) => {
		update((messages) => [...messages, event.data]);
	};

	return {
		subscribe,
		close: () => socket.close()
	};
}

export const notificationService = createNotificationStore();
