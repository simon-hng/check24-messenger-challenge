<script lang="ts">
	import '../app.css';
	import { browser } from '$app/environment';
	import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query';
	import FileUploadModal from '$lib/modals/fileUploadModal.svelte';
	import CreateEnquiryModal from '$lib/modals/createEnquiryModal.svelte';
	import { initializeStores, Modal, type ModalComponent } from '@skeletonlabs/skeleton';

	initializeStores();

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser
			}
		}
	});

	const modalRegistry: Record<string, ModalComponent> = {
		fileUpload: { ref: FileUploadModal },
		createEnquiry: { ref: CreateEnquiryModal }
	};
</script>

<QueryClientProvider client={queryClient}>
	<Modal components={modalRegistry} />
	<main>
		<slot />
	</main>
</QueryClientProvider>
