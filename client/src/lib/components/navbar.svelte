<script lang="ts">
	import { userStore } from '$lib/stores';
	import { faLock, faMessage, faQuestion } from '@fortawesome/free-solid-svg-icons';
	import { Avatar } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
</script>

<div class="p-4 border-b border-surface-500">
	<div class="flex items-center gap-3 justify-between">
		<div class="flex items-center gap-3">
			{#if $userStore}
				<Avatar src={$userStore.picture} width="w-12" rounded="rounded-full" />
				<h2 class="text-xl">{$userStore.name}</h2>
			{/if}
		</div>

		<nav class="flex">
			<a href="enquiry" class="btn gap-2">
				<Fa icon={faQuestion} />
				Enquiry</a
			>
			<a href="conversation" class="btn gap-2">
				<Fa icon={faMessage} />
				Conversation</a
			>
			<a href="auth" class="btn gap-2">
				<Fa icon={faLock} />
				Auth</a
			>
		</nav>
	</div>
</div>
