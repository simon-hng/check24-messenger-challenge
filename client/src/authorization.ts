import { GITHUB_ID } from "$env/static/private";
import { SvelteKitAuth } from "@auth/sveltekit";
import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

const authorization: Handle = async ({ event, resolve }) => {
    if (event.url.pathname.startsWith("/authenticated")) {
        const session = await event.locals.getSession();
        if (!session) {
            throw redirect(303, "/auth");
        }

        return resolve(event);
    }

    export const handle: Handle = sequence(
        SvelteKitAuth({ providers: [GitHub({ clientId: GITHUB_ID, clientSecret: GITHUB_SCRET })] }), authorization
    );
};

