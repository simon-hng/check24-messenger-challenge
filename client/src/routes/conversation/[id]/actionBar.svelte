<script lang="ts">
	import { conversationStore, userStore } from '$lib/stores';
	import FileListComponent from '$lib/components/fileList.svelte';
	import { faCheck, faHandshake, faXmark } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { api } from '$lib/api';
	import type { CurrentMessage } from '$lib/types';
	import { convertFileListToBase64Array } from '$lib/util/base64';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
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
			.post(`conversation/${dto.conversation.id}/message`, {
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
</script>

<div class="fixed bottom-0 w-full px-8 py-6">
	{#if $userStore?.account_type === 'ServiceProvider' && dto?.conversation?.state === 'Rejected'}
		<div class="card p-4 rounded-container-token flex flex-row items-center gap-4">
			<Fa icon={faXmark} size="2x" class="text-error-500" />
			<p>Your offer got rejected</p>
		</div>
	{:else if $userStore?.account_type === 'ServiceProvider' && dto?.conversation?.state === 'Accepted'}
		<div class="card p-4 rounded-container-token flex flex-row items-center gap-4">
			<Fa icon={faCheck} size="2x" class="text-success-500" />
			<p class="flex-grow">Your offer got accepted</p>
			<button class="btn variant-ghost">Request Review</button>
		</div>
	{:else}
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
