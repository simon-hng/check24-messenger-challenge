import type { Account } from './account';
import type { Message } from './message';

export interface Conversation {
	id: string; // TODO UUID
	state: 'Accepted' | 'Quoted' | 'Rejected';
	created_at: Date;
}

export interface ConversationDTO {
	conversation: Conversation;
	messages: Message[];
	partner: Account;
	unread_messages_count?: number;
}
