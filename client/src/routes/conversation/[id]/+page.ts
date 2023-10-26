import { api } from '$lib/api';
import type { Account } from '$lib/types/account';
import type { Conversation } from '$lib/types/conversation';

interface ConversationDTO {
	conversation: Conversation;
	participants: Account[];
}

export async function load({ params }: any) {
	const conversationId = params.id;

	const conversation = await api
		.get(`/conversation/${conversationId}`)
		.then((res) => res.data as ConversationDTO);

	return conversation;
}
