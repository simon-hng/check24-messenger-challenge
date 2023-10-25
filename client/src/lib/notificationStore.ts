import { get, writable } from 'svelte/store';
import { userStore } from './userStore';

const createNotificationStore = () => {
	const user = get(userStore);

	if (!user) {
		return;
	}

	const { subscribe, update } = writable<any[]>([]);

	const socket = new WebSocket('ws://localhost:8080/message/ws');

	socket.onmessage = (event) => {
		update((messages) => [...messages, event.data]);
	};

	const login = () => {
		socket.send(JSON.stringify({ type: 'Auth', id: user.name }));
	};

	return {
		subscribe,
		login,
		close: () => socket.close()
	};
};

export const notificationStore = createNotificationStore();
