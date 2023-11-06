<script lang="ts">
	import type { Account, Message } from '$lib/types';
	import { userStore } from '$lib/stores';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { formatDate } from '$lib/util/date';
	import { viewport } from '$lib/util/useViewportAction';
	import { api } from '$lib/api';
	import CheckIcon from '$lib/icons/checkIcon.svelte';
	import { onMount } from 'svelte';

	export let message: Message;
	export let partner: Account;

	let imageContainer: Element;

	const imageMimeTypes = ['image/jpeg', 'image/png'];
	let images = message?.attachments; //TODO: Filter mime type

	onMount(() => {
		images?.forEach((attachment) => {
			const img = new Image();
			img.src = attachment;

			imageContainer.appendChild(img);
		});
	});
</script>

{#if message.sender_id !== $userStore?.id}
	<div
		class="grid grid-cols-[auto_1fr] gap-2"
		use:viewport
		on:enterViewport={async () => {
			if (!message.read_at) {
				message = await api
					.post(`conversation/${message.conversation_id}/message/${message.id}/read`)
					.then((res) => res.data);
			}
		}}
	>
		<Avatar src={partner.picture} width="w-12" />
		<div class="card p-4 variant-soft rounded-tl-none space-y-2">
			<header class="flex justify-between items-center">
				<p class="font-bold">{partner.name}</p>
				<small class="opacity-50">{formatDate(message.created_at)}</small>
			</header>
			<p>{message.text}</p>

			<div bind:this={imageContainer} />

			{#if message.message_type === 'QuoteOffer'}
				<p class="italic">Accept or refuse this offer with a message</p>
			{/if}
		</div>
	</div>
{:else}
	<div class="grid grid-cols-[1fr_auto] gap-2">
		<div class="card p-4 rounded-tr-none space-y-2 bg-primary-200-700-token">
			<header class="flex justify-between items-center">
				<p class="font-bold">{$userStore.name}</p>
				<small class="opacity-50 flex gap-1 items-center">
					{formatDate(message.created_at)}
					{#if message.read_at}
						<CheckIcon class="h-4 w-4" />
					{/if}
				</small>
			</header>
			<p>{message.text}</p>

			<div bind:this={imageContainer} />

			{#if message.message_type === 'QuoteOffer'}
				<p class="italic">Waiting for partner response</p>
			{/if}
		</div>

		<Avatar src={$userStore?.picture} width="w-12" />
	</div>
{/if}
