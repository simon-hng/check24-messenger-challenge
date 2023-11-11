<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { conversationStore, userStore } from '$lib/stores';
	import { formatDate } from '$lib/util/date';
	import Fa from 'svelte-fa';
	import { faCheck } from '@fortawesome/free-solid-svg-icons';

	export let conversation_id: string;
	$: dto = $conversationStore ? $conversationStore[conversation_id] : undefined;
	$: messages = dto?.messages ?? [];
	$: lastMessage = messages[messages.length - 1];
	$: unreadCount = messages.filter(
		(msg) => !msg.read_at && msg.recipient_id === $userStore?.id
	).length;
</script>

<li>
	<a
		class="duration-300 w-full flex gap-3 p-2 bg-surface-hover-token rounded-2xl"
		href="conversation/{conversation_id}"
	>
		<Avatar
			src={dto?.partner?.picture}
			width="w-12 h-12"
			rounded="rounded-full"
			class="flex-shrink-0"
		/>
		<div class="overflow-hiden">
			<h3 class="font-semibold text-xl">{dto?.partner?.name}</h3>
			{#if lastMessage}
				<div class="flex items-center">
					{#if lastMessage.sender_id === $userStore?.id && lastMessage.read_at}
						<Fa icon={faCheck} size="sm" class="mr-2" />
					{/if}
					<p class="text-sm overflow-ellipsis whitespace-nowrap overflow-hidden w-full">
						{#if lastMessage.sender_id === $userStore?.id}
							You:
						{/if}
						{lastMessage.text ?? ''}
					</p>
				</div>
			{/if}
		</div>
		{#if lastMessage}
			<div class="flex flex-col items-end gap-1 ml-auto">
				<p class="text-sm whitespace-nowrap">{formatDate(lastMessage.created_at)}</p>
				{#if unreadCount}
					<span class="badge bg-primary-500">{unreadCount}</span>
				{/if}
			</div>
		{/if}
	</a>
</li>
