import { writable } from 'svelte/store';
import axios from 'axios';

export interface User {
	id: string; // TODO UUID
	name: string;
	picture: string;
	account_type: 'Customer' | 'ServiceProvider';
}

const createUserStore = () => {
	const { subscribe, set } = writable<User | undefined>();

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
		login,
		logout
	};
};

export const userStore = createUserStore();
