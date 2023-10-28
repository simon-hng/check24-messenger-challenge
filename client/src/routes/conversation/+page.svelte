<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { userStore } from '$lib/stores';
	import ChatRow from './chatRow.svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import { api } from '$lib/api';
	import type { ConversationDTO } from '$lib/types';

	const query = createQuery<ConversationDTO[], Error>({
		queryKey: ['conversations'],
		queryFn: async () => await api.get('conversation').then((res) => res.data)
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
			{:else if $query.isSuccess}
				{#each $query.data as conversationDTO}
					<ChatRow {conversationDTO} />
				{/each}
			{/if}
		</ul>
	</section>
</div>
