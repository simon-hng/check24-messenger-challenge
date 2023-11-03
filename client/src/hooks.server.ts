import { redirect, type Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

const authorization: Handle = async ({ event, resolve }) => {
	if (!event.url.pathname.startsWith('/auth')) {
		const session = event.cookies.get('id');
		if (!session) {
			throw redirect(303, '/auth');
		}
	}

	return resolve(event);
};

export const handle: Handle = sequence(authorization);
