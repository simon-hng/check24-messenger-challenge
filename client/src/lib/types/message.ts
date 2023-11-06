export interface Message {
	id: string; // UUID
	message_type: 'Standard' | 'AcceptQuote' | 'QuoteOffer';
	text: string;
	created_at: Date;
	read_at: Date;
	sender_id: string; // UUID
	recipient_id: string; // UUID
	conversation_id: string; //UUID
  attachments?: string[] // Base64 encoded files
}
