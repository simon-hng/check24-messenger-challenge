<script lang="ts">
	import type { ConversationDTO, Message } from '$lib/types';
	import { Avatar } from '@skeletonlabs/skeleton';
	import CheckIcon from '$lib/icons/checkIcon.svelte';
	import { userStore } from '$lib/stores';
	import { formatDate } from '$lib/util/date';

	export let conversationDTO: ConversationDTO;
	let { conversation, partner, messages, unread_messages_count } = conversationDTO;

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
				<div class="flex gap-2 items-center">
					{#if last_message.sender_id === $userStore.id}
						<div class="flex items-center">
							<CheckIcon class="h-4 w-4" />
							<CheckIcon class="h-4 w-4 -ml-2" />
						</div>
					{/if}
					<p class="text-sm overflow-ellipsis whitespace-nowrap overflow-hidden w-full">
						{last_message.text ?? ''}
					</p>
				</div>
			{/if}
		</div>
		<div class="flex flex-col items-end gap-1 ml-auto">
			<p class="text-sm whitespace-nowrap">{formatDate(last_message.created_at)}</p>
			{#if unread_messages_count}
				<span class="badge bg-primary-500">{unread_messages_count}</span>
			{/if}
		</div>
	</a>
</li>
