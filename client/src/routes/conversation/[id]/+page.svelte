<script lang="ts">
	import { Avatar, getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	import MessageBubble from './messageBubble.svelte';
	import { api } from '$lib/api';
	import { userStore, notificationStore } from '$lib/stores';
	import { onDestroy } from 'svelte';
	import { convertFileListToBase64Array } from '$lib/util/base64';
	import FileListComponent from '$lib/components/fileList.svelte';
	import type { Message } from '$lib/types';
	import { faCheck, faHandshake, faSadCry, faXmark } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let data;

	type CurrentMessage = Pick<Message, 'text' | 'message_type'> & { attachments?: FileList };

	let { conversation, partner, messages } = data;
	let currentMessage: CurrentMessage = {
		text: '',
		message_type: 'Standard'
	};

	const sendHandler = async () => {
		if (!currentMessage?.text?.length && !currentMessage?.attachments?.length) return;

		let attachments = currentMessage.attachments?.length
			? await convertFileListToBase64Array(currentMessage.attachments)
			: undefined;

		let message = await api
			.post(`conversation/${conversation.id}/message`, {
				message_type: currentMessage.message_type,
				text: currentMessage.text,
				sender_id: $userStore?.id,
				recipient_id: partner.id,
				conversation_id: conversation.id,
				attachments: attachments
			})
			.then((res) => res.data);

		message.attachments = attachments;

		messages = [...messages, message];
		currentMessage = {
			text: '',
			message_type: 'Standard'
		};
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

	<div class="fixed bottom-0 w-full px-8 py-6">
		{#if !(conversation.state === 'Rejected' && $userStore?.account_type === 'ServiceProvider')}
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
					<button on:click={() => console.log('TODO')}>
						<Fa icon={faCheck} />
					</button>
					<button on:click={() => console.log('TODO')}>
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
		{:else}
			<div class="card p-4 rounded-container-token flex flex-row items-center gap-4">
				<Fa icon={faXmark} size="2x" class="text-error-500" />
				<p>Your offer got rejected</p>
			</div>
		{/if}
	</div>
</div>
