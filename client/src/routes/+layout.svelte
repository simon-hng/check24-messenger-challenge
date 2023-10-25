<script lang="ts">
	import '../app.css';
	import axios from 'axios';
	import { browser } from '$app/environment';
	import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query';
	import { userStore } from '$lib/stores';
	import { Avatar } from '@skeletonlabs/skeleton';

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
		<header class="p-8 flex justify-between border-red-500 border-4">
			<div class="flex gap-3">
				<a href="/">home</a>
				<a href="/conversation">conv</a>
			</div>

			{#if $userStore}
				<Avatar src={$userStore?.picture} width="w-12" rounded="rounded-full" />
			{:else}
				<a href="/auth/login">login</a>
			{/if}
		</header>
		<slot />
	</main>
</QueryClientProvider>
