<script lang="ts">
	import { browser } from '$app/environment';
	import { beforeNavigate } from '$app/navigation';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';

	let currentMessage = '';
	let messages: any[] = [];
	export let data;

	let socket: WebSocket;
	if (browser) {
		socket = new WebSocket('ws://localhost:8080/conversation/ws');
		socket.onopen = () => {
			socket.send(`/join ${data.conversation_id}`);
		};
		socket.addEventListener('message', (event) => {
			messages = [...messages, event.data];
		});
	}

	beforeNavigate(() => {
		socket.close();
	});

	const sendHandler = () => {
		socket.send(currentMessage);
		messages = [...messages, currentMessage];
		currentMessage = '';
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
				src="https://i.pravatar.cc/?img=1"
				width="w-12 h-12"
				rounded="rounded-full"
				class="flex-shrink-0"
			/>
			<div>
				<h2 class="font-semibold text-xl">NAME</h2>
				<p class="text-sm">online</p>
			</div>
		</div>
	</div>

	<div class="overflow-y-auto">
		<div class="mx-8 flex flex-col gap-4">
			{#if messages.length}
				{#each messages as message}
					<MessageBubble {message} />
				{/each}
			{/if}
		</div>
	</div>

	<div class="m-8">
		<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] rounded-container-token">
			<button class="input-group-shim">+</button>
			<textarea
				bind:value={currentMessage}
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
