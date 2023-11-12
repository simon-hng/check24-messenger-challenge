import { writable } from 'svelte/store';
import { userStore } from './userStore';
import { browser } from '$app/environment';
import type { Account, WSNotification } from '$lib/types';

const createNotificationStore = () => {
	const { subscribe, set } = writable<WSNotification>();

	let socket: WebSocket;
  let user: Account | undefined;

  const startSocket = ()=> {
		socket = new WebSocket('ws://localhost:8080/notifications/ws');

		socket.onopen = () => {
			socket.send(JSON.stringify({ type: 'Auth', id: user?.id }));
		};
		socket.onmessage = (event) => {
			const notification = JSON.parse(event.data);
			set(notification);
		};
  }

	userStore.subscribe((newUser) => {
    user = newUser;
		if (browser && user) {
      startSocket();
    } else {
      socket?.close();
    }
	});

	return {
		subscribe,
	};
};

export const notificationStore = createNotificationStore();
