import axios from 'axios';
import { PUBLIC_API_BASE_URL } from '$env/static/public';

export const api = axios.create({
	baseURL: PUBLIC_API_BASE_URL,
	withCredentials: true,
	headers: { 'X-Custom-Header': 'foobar' }
});
