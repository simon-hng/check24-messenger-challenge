<script lang="ts">
	import { Avatar, ProgressRadial, getModalStore } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';
	import ActionBar from './actionBar.svelte';
	import { conversationStore } from '$lib/stores';
	import { api } from '$lib/api';
	import { navigating } from '$app/stores';
	import type { Message } from '$lib/types';

	export let data;

	$: dto = $conversationStore ? $conversationStore[data.id] : undefined;
	$: messages = dto?.messages ?? [];

	let hasPreviousMessages = true;
	let loadingPrevious: Promise<Message[]>;
	const limit = 10;

	const loadPreviousMessages = async () => {
		if (!messages?.length) return;

		loadingPrevious = api
			.get(`conversation/${data.id}/message`, {
				params: {
					limit,
					before: messages[0].created_at
				}
			})
			.then((res) => res.data);

		let previous = await loadingPrevious;

		if (!previous.length || previous.length < limit) {
			hasPreviousMessages = false;
		}

		$conversationStore[data.id].messages = [...previous, ...messages];
	};

	let marker: Element;
	const scrollToNewestMessage = () => {
		marker?.scrollIntoView({ behavior: 'smooth' });
	};

	$: if ($navigating) {
		setTimeout(scrollToNewestMessage, 1);
	}

	let scrollY: number;

	$: if (scrollY == 0 && hasPreviousMessages) {
		loadPreviousMessages();
	}
</script>

<svelte:window bind:scrollY />

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

<div class="mx-8 pt-32 flex flex-col gap-4">
	{#if messages.length && dto}
		{#if !hasPreviousMessages}
			<p class="text-sm text-center">Beginning of conversation</p>
		{/if}
		{#await loadingPrevious}
			<ProgressRadial class="mx-auto" width="w-10" />
		{/await}
		{#each messages as message}
			<MessageBubble {message} partner={dto.partner} />
		{/each}
	{/if}
</div>
<div bind:this={marker} />

<ActionBar conversation_id={data.id} on:messageSent={scrollToNewestMessage} />
