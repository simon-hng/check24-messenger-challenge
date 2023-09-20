<script lang="ts">
	import '../app.css';
	import axios from 'axios';
	import { browser } from '$app/environment';
	import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query';

	axios.defaults.baseURL = 'http://localhost:8080';
	axios.defaults.withCredentials = true;

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser
			}
		}
	});
</script>

<QueryClientProvider client={queryClient}>
	<main>
		<slot />
	</main>
</QueryClientProvider>
