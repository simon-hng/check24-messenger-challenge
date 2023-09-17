import type { Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

const authorization: Handle = async ({ event, resolve }) => {
	/*
    * TODO: Check cookie
	if (event.url.pathname.startsWith('/conversation')) {
		const session = await event.locals.getSession();
		if (!session) {
			throw redirect(303, '/auth');
		}
	}
  */

	return resolve(event);
};

export const handle: Handle = sequence(authorization);
