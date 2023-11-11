<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import Fa from 'svelte-fa';
	import { faUpload } from '@fortawesome/free-solid-svg-icons';

	import { FileDropzone, getModalStore } from '@skeletonlabs/skeleton';
	import FileListComponent from '$lib/components/fileList.svelte';

	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	// Local
	const modalStore = getModalStore();
	let files: FileList;

	// Handle Form Submission
	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(files);
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="modal-example-form card p-4 w-modal shadow-xl space-y-4">
		<FileDropzone name="files" bind:files multiple accept="image/png, image/jpeg, application/pdf">
			<svelte:fragment slot="lead">
				<Fa icon={faUpload} size="3x" class="mx-auto" />
			</svelte:fragment>
			<svelte:fragment slot="message">Attach a file or drag and drop</svelte:fragment>
			<svelte:fragment slot="meta">PNG, JPG, and PDF allowed</svelte:fragment>
		</FileDropzone>

		<FileListComponent {files} />

		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}
				>{parent.buttonTextCancel}</button
			>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Done</button>
		</footer>
	</div>
{/if}
