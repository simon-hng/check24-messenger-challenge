import axios from 'axios';

export async function load({ params }: any) {
	const conversationId = params.id;

	const conversation: Conversation = await axios
		.get(`/conversation/${conversationId}`)
		.then((res) => res.data);

	return conversation;
}
