<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';
	import ActionBar from './actionBar.svelte';
	import { conversationStore } from '$lib/stores';
	import { api } from '$lib/api';

	export let data;

	$: dto = $conversationStore ? $conversationStore[data.id] : undefined;
	$: messages = dto?.messages ?? [];

	const loadPreviousMessages = async () => {
		const previous = await api
			.get(`conversation/${data.id}/message`, {
				params: {
					limit: 10,
					before: messages[0].created_at
				}
			})
			.then((res) => res.data);

		$conversationStore[data.id].messages = [...previous, ...messages];
	};
</script>

<div class="h-screen">
	<div class="top-0 fixed w-full z-10">
		<div class="rounded-b-2xl w-full p-4 flex flex-row items-center gap-3 bg-surface-100-800-token">
			<a href="/conversation" class="bg-surface-hover-token p-2 rounded-full duration-300">
				<Icon src={ArrowLeft} size="24" />
			</a>
			<div class="flex flex-row gap-2">
				<Avatar
					src={dto?.partner?.picture}
					width="w-12 h-12"
					rounded="rounded-full"
					class="flex-shrink-0"
				/>
				<div>
					<h2 class="font-semibold text-xl">{dto?.partner?.name}</h2>
					<p class="text-sm">{dto?.partner?.account_type}</p>
				</div>
			</div>
		</div>
	</div>

	<div class="mx-8 py-32 flex flex-col gap-4">
		{#if messages.length && dto}
			<button on:click={loadPreviousMessages} class="btn variant-filled">Load more</button>
			{#each messages as message}
				<MessageBubble {message} partner={dto.partner} />
			{/each}
		{/if}
	</div>

	<ActionBar conversation_id={data.id} />
</div>
