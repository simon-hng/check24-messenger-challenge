<script lang="ts">
	import { faker } from '@faker-js/faker';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { createQuery } from '@tanstack/svelte-query';
	import type { Chat } from './types';
	import axios from 'axios';
	import Chatrow from './chatrow.svelte';

	const query = createQuery<Chat[], Error>({
		queryKey: ['conversations'],
		queryFn: async () => await axios.get('/conversation').then((res) => res.data)
	});
</script>

<div class="h-screen">
	<div class="p-4 border-b border-surface-500">
		<div class="flex items-center gap-3">
			<Avatar
				src="https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop"
				width="w-12"
				rounded="rounded-full"
			/>
			<h2 class="text-xl">Chats</h2>
		</div>
	</div>

	<section>
		<ul class="list p-2">
			{#if $query.isLoading}
				loading...
			{:else if $query.error}
				error happened
				{$query.error}
			{:else if $query.isSuccess}
				{#each $query.data as chat}
					<Chatrow
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
