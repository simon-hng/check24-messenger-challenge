<script lang="ts">
	import type { SvelteComponent } from 'svelte';

	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();

	// Form Data
	const formData = {
		title: '',
		description: ''
	};

	// We've created a custom submit function to pass the response and close the modal.
	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="card p-4 w-modal shadow-xl space-y-4">
		<header class="text-2xl font-bold">Create enquiry</header>
		<form class="space-y-4">
			<label class="label">
				<span>Name</span>
				<input class="input p-2" type="text" bind:value={formData.title} placeholder="Title" />
			</label>
			<label class="label">
				<span>Description</span>
				<textarea
					class="textarea p-2"
					rows="4"
					bind:value={formData.description}
					placeholder="Description"
				/>
			</label>
		</form>

		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}
				>{parent.buttonTextCancel}</button
			>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Submit</button>
		</footer>
	</div>
{/if}
