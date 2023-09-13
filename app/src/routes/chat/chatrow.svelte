<script lang="ts">
	import type { Chat } from './+page';
	import { Avatar } from '@skeletonlabs/skeleton';

	export let chat: Chat;

	let last_message_string = '10:15 AM';
</script>

<li>
	<a class="w-full flex gap-3 p-2" href="chat/{chat.id}">
		<Avatar src={chat.avatar} width="w-12 h-12" rounded="rounded-full" class="flex-shrink-0" />
		<div class="overflow-hidden">
			<h3 class="font-semibold text-xl">{chat.name}</h3>
			<p class="text-sm overflow-ellipsis whitespace-nowrap overflow-hidden w-full">
				{chat.last_message}
			</p>
		</div>
		<div class="flex flex-col items-end gap-1 ml-auto">
			<p class="text-sm whitespace-nowrap">{last_message_string}</p>
			{#if chat.count_unread}
				<span class="badge bg-primary-500">{chat.count_unread}</span>
			{/if}
		</div>
	</a>
</li>
