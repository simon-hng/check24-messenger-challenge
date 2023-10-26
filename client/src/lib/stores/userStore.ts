import { localStore } from './localStore';
import { browser } from '$app/environment';
import { writable } from 'svelte/store';
import { api } from '$lib/api';
import type { Account } from '$lib/types/account';

const createUserStore = () => {
	const { subscribe, set } = browser ? localStore('auth') : writable();

	const login = async (username: string) => {
		await api
			.post('/auth/login', {
				account_name: username
			})
			.then((res) => set(res.data as Account))
			.catch((err) => console.error(err));
	};

	const logout = async () => {
		await api.post('/auth/logout').then(() => set(undefined));
	};

	return {
		subscribe,
		set,
		login,
		logout
	};
};

export const userStore = createUserStore();
