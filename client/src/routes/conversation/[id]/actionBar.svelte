<script lang="ts">
	import { conversationStore, userStore } from '$lib/stores';
	import FileListComponent from '$lib/components/fileList.svelte';
	import {
		faCheck,
		faHandshake,
		faStar as faStarFilled,
		faXmark
	} from '@fortawesome/free-solid-svg-icons';
	import { faStar } from '@fortawesome/free-regular-svg-icons';
	import Fa from 'svelte-fa';
	import { api } from '$lib/api';
	import type { CurrentMessage } from '$lib/types';
	import { convertFileListToBase64Array } from '$lib/util/base64';
	import { getModalStore, Ratings, type ModalSettings } from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';

	export let conversation_id: string;

	$: dto = $conversationStore ? $conversationStore[conversation_id] : undefined;
	let currentMessage: CurrentMessage = {
		text: '',
		message_type: 'Standard'
	};

	const dispatch = createEventDispatcher();

	const sendHandler = async () => {
		if ((!currentMessage?.text?.length && !currentMessage?.attachments?.length) || !dto) return;

		let attachments = currentMessage.attachments?.length
			? await convertFileListToBase64Array(currentMessage.attachments)
			: undefined;

		let message = await api
			.post(`/conversation/${dto.conversation.id}/message`, {
				message_type: currentMessage.message_type,
				text: currentMessage.text,
				sender_id: $userStore?.id,
				recipient_id: dto.partner.id,
				conversation_id: dto.conversation.id,
				attachments: attachments
			})
			.then((res) => res.data);

		message.attachments = attachments;

		$conversationStore[conversation_id].messages = [...dto.messages, message];

		if (
			currentMessage.message_type === 'AcceptQuote' ||
			currentMessage.message_type === 'RejectQuote'
		) {
			$conversationStore[conversation_id].conversation.state =
				currentMessage.message_type === 'AcceptQuote' ? 'Accepted' : 'Rejected';
		}

		currentMessage = {
			text: '',
			message_type: 'Standard'
		};

		// Wait a bit for messages to update
		setTimeout(() => dispatch('messageSent'), 1);
	};

	const modalStore = getModalStore();
	const openFileUploadModal = () => {
		const modal: ModalSettings = {
			type: 'component',
			component: 'fileUpload',
			response: async (files: FileList) => {
				currentMessage.attachments = files;
			}
		};
		modalStore.trigger(modal);
	};

	let value = {
		current: 0,
		max: 5
	};

	let clientHeight: number;
	let placeholder: Element;

	$: if (clientHeight) {
		placeholder?.setAttribute('style', `height: ${clientHeight}px`);
	}
</script>

<div bind:this={placeholder} />

<div class="fixed bottom-0 w-full px-8 py-6" bind:clientHeight>
	{#if $userStore?.account_type === 'ServiceProvider' && dto?.conversation?.state === 'Rejected'}
		<div class="card p-4 rounded-container-token flex flex-row items-center gap-4">
			<Fa icon={faXmark} size="2x" class="text-error-500" />
			<p>Your offer got rejected</p>
		</div>
	{:else if $userStore?.account_type === 'ServiceProvider' && dto?.conversation?.state === 'Accepted'}
		<div class="card p-4 rounded-container-token flex flex-row items-center gap-4">
			<Fa icon={faCheck} size="2x" class="text-success-500" />
			<p class="flex-grow">Your offer got accepted</p>
		</div>

		{#if dto.review}
			<div class="card p-4 rounded-container-token flex flex-row items-center gap-4">
				<Fa icon={faCheck} size="2x" class="text-success-500" />
				<p class="flex-grow">{dto.partner.name} gave you a review!</p>
				<Ratings value={dto.review.score} max={value.max}>
					<svelte:fragment slot="empty"><Fa icon={faStar} /></svelte:fragment>
					<svelte:fragment slot="full"><Fa icon={faStarFilled} /></svelte:fragment>
				</Ratings>
			</div>
		{/if}
	{:else if $userStore?.account_type === 'Customer' && dto?.conversation?.state === 'Accepted'}
		<div class="card p-4 rounded-container-token flex gap-4 flex-col">
			{#if !dto.review}
				<p class="text-center">
					Leave a review for {dto.partner.name}
				</p>
				<Ratings
					bind:value={value.current}
					max={value.max}
					interactive
					on:icon={(event) => {
						value.current = event.detail.index;
					}}
				>
					<svelte:fragment slot="empty"><Fa icon={faStar} /></svelte:fragment>
					<svelte:fragment slot="full"><Fa icon={faStarFilled} /></svelte:fragment>
				</Ratings>
				<button
					class="btn variant-ghost"
					on:click={() => {
						api
							.post('/review', {
								reviewer_id: $userStore?.id,
								recipient_id: dto?.partner?.id,
								score: value.current
							})
							.then((res) => res.data)
							.then((review) => {
								$conversationStore[conversation_id].review = review;
							});
					}}>Submit Review</button
				>
			{:else}
				<p class="text-center">
					You left a review for {dto.partner.name} of
					{dto.review.score} / {value.max}
				</p>
				<Ratings value={dto.review.score} max={value.max}>
					<svelte:fragment slot="empty"><Fa icon={faStar} /></svelte:fragment>
					<svelte:fragment slot="full"><Fa icon={faStarFilled} /></svelte:fragment>
				</Ratings>
			{/if}
		</div>
	{:else if dto?.conversation?.state === 'Quoted'}
		{#if currentMessage.attachments}
			<div class="bg-surface-200-700-token mb-4 rounded p-4">
				<FileListComponent files={currentMessage.attachments} />
			</div>
		{/if}

		<div class="input-group input-group-divider rounded-container-token flex flex-row">
			<button class="input-group-shim" on:click={openFileUploadModal}>+</button>

			{#if $userStore?.account_type === 'ServiceProvider'}
				<button
					class={currentMessage.message_type === 'Standard'
						? 'duration-200'
						: 'variant-soft-success rounded duration-200'}
					on:click={() => {
						currentMessage.message_type =
							currentMessage.message_type === 'Standard' ? 'QuoteOffer' : 'Standard';
					}}
				>
					<Fa icon={faHandshake} />
				</button>
			{:else}
				<button
					class={currentMessage.message_type !== 'AcceptQuote'
						? 'duration-200'
						: 'variant-soft-success rounded duration-200'}
					on:click={() => {
						currentMessage.message_type =
							currentMessage.message_type === 'Standard' ? 'AcceptQuote' : 'Standard';
					}}
				>
					<Fa icon={faCheck} />
				</button>
				<button
					class={currentMessage.message_type !== 'RejectQuote'
						? 'duration-200'
						: 'variant-soft-error rounded duration-200'}
					on:click={() => {
						currentMessage.message_type =
							currentMessage.message_type === 'Standard' ? 'RejectQuote' : 'Standard';
					}}
				>
					<Fa icon={faXmark} />
				</button>
			{/if}
			<textarea
				bind:value={currentMessage.text}
				class="bg-transparent border-0 ring-0 p-2 resize-none flex-grow"
				name="prompt"
				id="prompt"
				placeholder="Write a message..."
				rows="1"
			/>
			<button class="variant-filled-primary" type="submit" on:click={sendHandler}>Send</button>
		</div>
	{/if}
</div>
