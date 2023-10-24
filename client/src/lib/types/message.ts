export interface Message {
	message_type: 'Standard' | 'Accept_quote' | 'Quote_offer';
	text: string;
	created_at: Date;
	sender_id: string; // UUID,
	recipient_id: string; // UUID
	conversation_id: string; //UUID
}