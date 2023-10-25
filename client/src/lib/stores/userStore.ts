import axios from 'axios';
import { localStore } from './localStore';
import { browser } from '$app/environment';
import { writable } from 'svelte/store';

export interface User {
	id: string; // TODO UUID
	name: string;
	picture: string;
	account_type: 'Customer' | 'ServiceProvider';
}

const createUserStore = () => {
	const { subscribe, set} = browser ? localStore("auth") : writable();

	const login = async (username: string) => {
		await axios
			.post('/auth/login', {
				account_name: username
			})
			.then((res) => set(res.data))
			.catch((err) => console.error(err));
	};

	const logout = async () => {
		await axios.post('/auth/logout').then(() => set(undefined));
	};

	return {
		subscribe,
    set,
		login,
		logout
	};
};

export const userStore = createUserStore();
