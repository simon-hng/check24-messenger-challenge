import { api } from '$lib/api';
import type { ConversationDTO } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
  return params;
};
