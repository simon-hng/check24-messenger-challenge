<script lang="ts">
	import { api } from '$lib/api';
	import { userStore } from '$lib/stores';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';
	import { Avatar, getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import Fa from 'svelte-fa';
	import { createQuery } from '@tanstack/svelte-query';

	const modalStore = getModalStore();
	const openCreateEnquiryModal = () => {
		const modal: ModalSettings = {
			type: 'component',
			component: 'createEnquiry',
			response: async (formData) => {
				await api.post('enquiry', {
					...formData,
					enquirer_id: $userStore?.id
				});

				$query.refetch();
			}
		};
		modalStore.trigger(modal);
	};

	const query = createQuery({
		queryKey: ['enquiry'],
		queryFn: () => api.get('enquiry').then((res) => res.data)
	});
</script>

<div class="p-4 border-b border-surface-500 flex justify-between items-center">
	<div class="flex items-center gap-3">
		{#if $userStore?.picture}
			<Avatar src={$userStore?.picture} width="w-12" rounded="rounded-full" />
		{/if}
		<h2 class="text-xl">Enquiries</h2>
	</div>
</div>

<section>
	<ul class="list p-2">
		{#if $query.isLoading}
			<div class="card placeholder h-32 rounded" />
			<div class="card placeholder h-32 rounded" />
			<div class="card placeholder h-32 rounded" />
		{:else if $query.isSuccess}
			{#each $query.data as enquiry}
				<div class="card">
					<header class="card-header">
						<h2 class="text-2xl mb-3">{enquiry.title}</h2>
					</header>
					<section class="p-4">
						<p>{enquiry.description}</p>
					</section>
					<hr class="opacity-50" />
					<footer class="card-footer pt-4 flex justify-between items-center">
						<div class="flex gap-3">
							<p>Enquiry created by {enquiry.enquirer_id}</p>
						</div>
						{#if enquiry.enquirer_id !== $userStore?.id}
							<button
								class="btn variant-filled"
								on:click={() => {
									api
										.post('conversation', {
											partner_id: enquiry.enquirer_id
										})
										.then((res) => {
											// TODO: invalidate conversationStore
											goto(`conversation/${res.data.id}`);
										});
								}}>Contact</button
							>
						{/if}
					</footer>
				</div>
			{/each}
		{/if}
	</ul>
</section>

<button class="mx-2 btn variant-soft" on:click={openCreateEnquiryModal}>
	<Fa icon={faPlus} class="mr-2" />
	Create new Enquiry
</button>
