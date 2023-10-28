<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { userStore } from '$lib/stores';
	import { conversationStore } from '$lib/stores/conversationStore';
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
			{#each $conversationStore as conversationDTO}
				<ChatRow {conversationDTO} />
			{/each}
		</ul>
	</section>
</div>
