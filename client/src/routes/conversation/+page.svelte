<script lang="ts">
	import { conversationStore } from '$lib/stores';
	import ChatRow from './chatRow.svelte';
	import Navbar from '$lib/components/navbar.svelte';
</script>

<Navbar />

<section>
	<ul class="list p-2">
		{#if $conversationStore}
			{#each Object.keys($conversationStore) as conversation_id}
				<ChatRow {conversation_id} />
			{/each}
		{/if}
	</ul>
</section>
