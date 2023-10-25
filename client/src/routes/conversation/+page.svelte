<script lang="ts">
	import { faker } from '@faker-js/faker';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { createQuery } from '@tanstack/svelte-query';
	import type { Chat } from './types';
	import axios from 'axios';
	import ChatRow from './chatRow.svelte';
	import { userStore } from '$lib/stores';

	const query = createQuery<Chat[], Error>({
		queryKey: ['conversations'],
		queryFn: async () => await axios.get('/conversation/').then((res) => res.data)
	});
</script>

<div class="h-screen">
	<div class="p-4 border-b border-surface-500">
		<div class="flex items-center gap-3">
			{#if $userStore?.picture}
				<Avatar src={$userStore?.picture} width="w-12" rounded="rounded-full" />
			{/if}
			<h2 class="text-xl">Chats</h2>
		</div>
	</div>

	<section>
		<ul class="list p-2">
			{#if $query.isLoading}
				<div class="w-full h-16 placeholder animate-pulse rounded-xl" />
				<div class="w-full h-16 placeholder animate-pulse rounded-xl" />
				<div class="w-full h-16 placeholder animate-pulse rounded-xl" />
			{:else if $query.error}
				error happened
				{$query.error}
			{:else if $query.isSuccess}
				{#each $query.data as chat}
					<ChatRow
						chat={{
							id: chat.id,
							avatar: faker.image.avatar(),
							name: chat.name,
							last_message: faker.lorem.text(),
							updated_at: chat.updated_at,
							count_unread: chat.count_unread
						}}
					/>
				{/each}
			{:else}
				????
			{/if}
		</ul>
	</section>
</div>
