<script lang="ts">
	import { userStore } from '$lib/stores';
	import { Avatar } from '@skeletonlabs/skeleton';
</script>

<div class="p-4 border-b border-surface-500">
	<div class="flex items-center gap-3 justify-between">
		<div class="flex items-center gap-3">
			{#if $userStore}
				<Avatar src={$userStore.picture} width="w-12" rounded="rounded-full" />
				<h2 class="text-xl">{$userStore.name}</h2>
			{/if}
		</div>

		<nav class="flex gap-3">
			<a href="enquiry">enquiry</a>
			<a href="conversation">conversation</a>
			<a href="auth">auth</a>
		</nav>
	</div>
</div>
