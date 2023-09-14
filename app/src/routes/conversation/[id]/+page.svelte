<script lang="ts">
	import { websocket } from '@sveu/browser';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';

	let currentMessage = '';
	let messages: any[] = [];
	const { data, send } = websocket('ws://localhost:8080/conversation/ws');
	data.subscribe((newMessage) => (messages = [...messages, newMessage]));
</script>

<div class="rounded-b-2xl p-4 flex flex-row items-center gap-3 top-0 left-0 w-full">
	<a href="/conversation">
		<Icon src={ArrowLeft} size="24" />
	</a>
	<div class="flex flex-row gap-2">
		<Avatar
			src="https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop"
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

{#if messages.length}
	{#each messages as message}
		<p>{message}</p>
	{/each}
{/if}

<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] rounded-container-token">
	<button class="input-group-shim">+</button>
	<textarea
		bind:value={currentMessage}
		class="bg-transparent border-0 ring-0"
		name="prompt"
		id="prompt"
		placeholder="Write a message..."
		rows="1"
	/>
	<button
		class="variant-filled-primary"
		type="submit"
		on:click={() => {
			send(currentMessage);
		}}>Send</button
	>
</div>
