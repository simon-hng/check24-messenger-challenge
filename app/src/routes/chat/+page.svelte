<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import Chatrow from './chatrow.svelte';
	export let data;
</script>

<div class="h-screen bg-white">
	<div class="p-4 border-b border-surface">
		<div class="flex items-center gap-3">
			<Avatar
				src="https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop"
				width="w-12"
				rounded="rounded-full"
			/>
			<h2 class="text-xl">Chats</h2>
		</div>
	</div>

	<section>
		<ul class="list p-2">
			{#each data.chats as chat}
				<Chatrow {chat} />
			{/each}
		</ul>
	</section>
</div>
