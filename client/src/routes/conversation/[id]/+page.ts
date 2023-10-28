import { api } from '$lib/api';
import type { ConversationDTO } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const { id } = params;

	const conversationDTO = api.get(`/conversation/${id}`).then((res) => res.data as ConversationDTO);

	return conversationDTO;
};
