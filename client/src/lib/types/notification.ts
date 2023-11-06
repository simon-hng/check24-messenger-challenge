import type { Message } from './message';

export type WSNotification =
	| ({ type: 'Message' } & Message)
	| ({ type: 'Read' } & NotifyRead)

interface NotifyRead {
	message_id: string;
	read_at: Date;
	sender_id: string;
	conversation_id: string;
}
