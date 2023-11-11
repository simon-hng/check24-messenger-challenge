import { api } from '$lib/api';
import type { ConversationDTO, WSNotification } from '$lib/types';
import { writable } from 'svelte/store';
import { notificationStore } from './notificationStore';

export const createConversationStore = () => {
	const store = writable<Record<string, ConversationDTO>>();

	const fetchConversations = () =>
		api
			.get('conversation')
			.then((res) => res.data)
			.then((dtos: ConversationDTO[]) => {
				let conversationMap: Record<string, ConversationDTO> = {};
				for (const dto of dtos) {
					const conversationId = dto.conversation.id;
					conversationMap[conversationId] = dto;
				}
				store.set(conversationMap);
			})
			.catch((err) => console.error(err));

	fetchConversations();

	const notifyMessageRead = (notification: Extract<WSNotification, { type: 'Read' }>) => {
		store.update((conversations) => {
			let messages = conversations[notification.conversation_id].messages;
			messages = messages.map((message) => {
				return message.id === notification.message_id
					? { ...message, read_at: notification.read_at }
					: message;
			});
			conversations[notification.conversation_id].messages = messages;

			return conversations;
		});
	};

	notificationStore.subscribe((notification) => {
		if (notification?.type === 'Message') {
			store.update((conversations) => {
				let messages = conversations[notification.conversation_id].messages;
				messages = [...messages, notification];
				conversations[notification.conversation_id].messages = messages;

				if (notification.message_type === 'RejectQuote') {
					conversations[notification.conversation_id].conversation.state = 'Rejected';
				}
				if (notification.message_type === 'AcceptQuote') {
					conversations[notification.conversation_id].conversation.state = 'Accepted';
				}

				return conversations;
			});
		}

		if (notification?.type === 'Read') {
			notifyMessageRead(notification);
		}
	});

	return {
		set: store.set,
		update: store.update,
		subscribe: store.subscribe,
		fetch: fetchConversations
	};
};

export const conversationStore = createConversationStore();
