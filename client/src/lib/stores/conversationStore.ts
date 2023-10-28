import { api } from '$lib/api';
import type { ConversationDTO } from '$lib/types';
import { writable } from 'svelte/store';

const createConversationStore = () => {
	const { subscribe, set } = writable<ConversationDTO>([] as any);

	api
		.get('conversation')
		.then((response) => {
			set(response.data);
		})
		.catch((error) => {
			console.error(error);
		});

	return {
		subscribe,
		set
	};
};

export const conversationStore = createConversationStore();
