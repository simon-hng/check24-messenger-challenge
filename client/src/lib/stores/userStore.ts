import { localStore } from './localStore';
import { browser } from '$app/environment';
import { writable } from 'svelte/store';
import { api } from '$lib/api';
import type { Account } from '$lib/types';
import type { Writable } from 'svelte/store';

const createUserStore = () => {
	const store: Writable<Account | undefined> = browser ? localStore('auth') : writable();

	const login = async (username: string): Promise<any> => {
		return api
			.post('/auth/login', {
				account_name: username
			})
			.then((res) => store.set(res.data));
	};

	const logout = async () => {
		await api.post('/auth/logout');
		store.update(() => undefined);
	};

	return {
		subscribe: store.subscribe,
		set: store.set,
		login,
		logout
	};
};

export const userStore = createUserStore();
