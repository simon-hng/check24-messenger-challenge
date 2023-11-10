import type { Review } from './review';
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
  review: Review;
}
