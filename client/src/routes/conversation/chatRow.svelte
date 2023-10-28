<script lang="ts">
	import type { ConversationDTO, Message } from '$lib/types';
	import { Avatar } from '@skeletonlabs/skeleton';
	import CheckIcon from '$lib/icons/checkIcon.svelte';
	import { userStore } from '$lib/stores';

	export let conversationDTO: ConversationDTO;
	let { conversation, partner, messages } = conversationDTO;

	let last_message: Message = messages[messages.length - 1];
</script>

<li>
	<a
		class="duration-300 w-full flex gap-3 p-2 bg-surface-hover-token rounded-2xl"
		href="conversation/{conversation.id}"
	>
		<Avatar src={partner.picture} width="w-12 h-12" rounded="rounded-full" class="flex-shrink-0" />
		<div class="overflow-hiden">
			<h3 class="font-semibold text-xl">{partner.name}</h3>
			{#if last_message}
				<div class="flex gap-2">
					{#if last_message.sender_id === $userStore.id}
						<CheckIcon />
					{/if}
					<p class="text-sm overflow-ellipsis whitespace-nowrap overflow-hidden w-full">
						{last_message.text ?? ''}
					</p>
				</div>
			{/if}
		</div>
		<div class="flex flex-col items-end gap-1 ml-auto">
			<p class="text-sm whitespace-nowrap">{last_message.created_at}</p>
			{#if messages}
				<span class="badge bg-primary-500">2</span>
			{/if}
		</div>
	</a>
</li>
