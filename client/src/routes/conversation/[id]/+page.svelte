<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';
	import ActionBar from './actionBar.svelte';
	import { api } from '$lib/api';
	import { notificationStore } from '$lib/stores';
	import { onDestroy } from 'svelte';

	export let data;

	let { conversation, partner, messages } = data;

	const unsubscribe = notificationStore.subscribe((notification) => {
		switch (notification?.type) {
			case 'Message': {
				messages = [...(messages ?? []), notification];
				if (notification.message_type === 'RejectQuote') {
					conversation.state = 'Rejected';
				}
				return;
			}
			case 'Read': {
				messages = messages.map((message) => {
					if (message.id === notification.message_id) {
						return { ...message, read_at: notification.read_at };
					}
					return message;
				});

				return;
			}
			case 'Confirm_auth': {
				console.log('authenticated ');
				return;
			}
		}
	});

	onDestroy(unsubscribe);

	const loadPreviousMessages = async () => {
		const previous = await api
			.get(`conversation/${conversation.id}/message`, {
				params: {
					limit: 10,
					before: messages[0].created_at
				}
			})
			.then((res) => res.data);

		messages = [...previous, ...messages];
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
					src={partner?.picture}
					width="w-12 h-12"
					rounded="rounded-full"
					class="flex-shrink-0"
				/>
				<div>
					<h2 class="font-semibold text-xl">{partner?.name}</h2>
					<p class="text-sm">{partner?.account_type}</p>
				</div>
			</div>
		</div>
	</div>

	<div class="mx-8 py-32 flex flex-col gap-4">
		{#if messages?.length}
			<button on:click={loadPreviousMessages} class="btn variant-filled">Load more</button>
			{#each messages as message}
				<MessageBubble {message} {partner} />
			{/each}
		{/if}
	</div>

	<ActionBar {data} />
</div>
