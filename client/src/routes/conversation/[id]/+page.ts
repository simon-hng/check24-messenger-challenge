import { api } from '$lib/api';
import type { ConversationDTO } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	const { id } = params;

	const conversationDTO: ConversationDTO = await api
		.get(`/conversation/${id}`)
		.then((res) => res.data)
		.catch((err) => console.error(err));

	return conversationDTO;
};
