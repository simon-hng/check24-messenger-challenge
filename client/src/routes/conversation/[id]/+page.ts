import axios from 'axios';

interface Conversation {
	id: string; // TODO UUID
	state: 'Accepted' | 'Quoted' | 'Rejected';
	created_at: Date;
}

export async function load({ params }: any) {
	const conversationId = params.id;

	return (await axios
		.get(`/conversation/${conversationId}`)
		.then((res) => res.data)) as Conversation;
}
