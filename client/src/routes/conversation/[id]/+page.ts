import { api } from '$lib/api';
import type { Conversation } from '$lib/types/conversation';

export async function load({ params }: any) {
	const conversationId = params.id;

	const conversation: Conversation = await api
		.get(`/conversation/${conversationId}`, {params: {
      includeMessages: 50
    }})
		.then((res) => res.data);

	return conversation;
}
