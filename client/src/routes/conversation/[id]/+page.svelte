<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';
	import { api } from '$lib/api';
	import { userStore } from '$lib/stores';

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
</script>

<div class="grid grid-rows-[auto_1fr_auto] h-screen">
	<div
		class="rounded-b-2xl p-4 flex flex-row items-center gap-3 top-0 left-0 w-full bg-surface-100-800-token mb-8"
	>
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

	<div class="overflow-y-auto">
		<div class="mx-8 flex flex-col gap-4">
			{#if messages.length}
				{#each messages as message}
					<MessageBubble {message} {partner} />
				{/each}
			{/if}
		</div>
	</div>

	<div class="m-8">
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
