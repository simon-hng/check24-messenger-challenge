<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';
	import { api } from '$lib/api';
	import { userStore, notificationStore } from '$lib/stores';
	import { onDestroy } from 'svelte';

	export let data;

	let { conversation, partner, messages } = data;
	let currentMessageText = '';

	const sendHandler = async () => {
		let message = await api
			.post(`conversation/${conversation.id}/message`, {
				message_type: 'Standard',
				text: currentMessageText,
				sender_id: $userStore.id,
				recipient_id: partner.id,
				conversation_id: conversation.id
			})
			.then((res) => res.data);

		messages = [...messages, message];
		currentMessageText = '';
	};

	const unsubscribe = notificationStore.subscribe((notification) => {
		switch (notification?.type) {
			case 'Message': {
				messages = [...(messages ?? []), notification];
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

	let scrollY: number;
	let innerHeight: number;
	let outerHeight: number;

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

	let loadingPrevious: Promise<any>;
	/*
	$: if (scrollY === 0 && messages.length) {
		loadingPrevious = loadPreviousMessages();
	}
  */
</script>

<svelte:window bind:scrollY bind:innerHeight bind:outerHeight />

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
					<p class="text-sm">online</p>
				</div>
			</div>
		</div>
	</div>

	<div class="mx-8 py-32 flex flex-col gap-4">
		{#if messages?.length}
			{#await loadingPrevious}
				loading
			{:then}
				<button on:click={loadPreviousMessages}>Load more</button>
			{/await}
			{#each messages as message}
				<MessageBubble {message} {partner} />
			{/each}
		{/if}
	</div>

	<div class="fixed bottom-0 w-full px-8 py-6">
		<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] rounded-container-token">
			<button class="input-group-shim">+</button>
			<textarea
				bind:value={currentMessageText}
				class="bg-transparent border-0 ring-0 p-2 resize-none"
				name="prompt"
				id="prompt"
				placeholder="Write a message..."
				rows="1"
			/>
			<button class="variant-filled-primary" type="submit" on:click={sendHandler}>Send</button>
		</div>
	</div>
</div>
