<script lang="ts">
	import type { ConversationDTO } from '$lib/types';
	import { Avatar } from '@skeletonlabs/skeleton';

	export let conversationDTO: ConversationDTO;
	let { conversation, partner, messages } = conversationDTO;

	let last_message_string = '10:15 AM';
</script>

<li>
	<a
		class="duration-300 w-full flex gap-3 p-2 bg-surface-hover-token rounded-2xl"
		href="conversation/{conversation.id}"
	>
		<Avatar src={partner.picture} width="w-12 h-12" rounded="rounded-full" class="flex-shrink-0" />
		<div class="overflow-hiden">
			<h3 class="font-semibold text-xl">{partner.name}</h3>
			<p class="text-sm overflow-ellipsis whitespace-nowrap overflow-hidden w-full">
				{messages[0] ?? ''}
			</p>
		</div>
		<div class="flex flex-col items-end gap-1 ml-auto">
			<p class="text-sm whitespace-nowrap">{last_message_string}</p>
			{#if messages}
				<span class="badge bg-primary-500">2</span>
			{/if}
		</div>
	</a>
</li>
