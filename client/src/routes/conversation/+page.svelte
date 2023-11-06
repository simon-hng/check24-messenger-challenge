<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { userStore, conversationStore } from '$lib/stores';
	import ChatRow from './chatRow.svelte';
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
			{#if $conversationStore}
				{#each Object.keys($conversationStore) as conversation_id}
					<ChatRow {conversation_id} />
				{/each}
			{/if}
		</ul>
	</section>
</div>
