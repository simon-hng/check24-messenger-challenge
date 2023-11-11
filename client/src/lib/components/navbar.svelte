<script lang="ts">
	import { goto } from '$app/navigation';
	import { userStore } from '$lib/stores';
	import {
		faMessage,
		faQuestion,
		faRightFromBracket,
		faRightToBracket
	} from '@fortawesome/free-solid-svg-icons';
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
			{#if $userStore}
				<a href="enquiry" class="btn gap-2">
					<Fa icon={faQuestion} />
					Enquiry</a
				>
				<a href="conversation" class="btn gap-2">
					<Fa icon={faMessage} />
					Conversation</a
				>
				<button
					on:click={async () => {
						await userStore.logout();
						goto('/auth');
					}}
					class="btn gap-2"
				>
					<Fa icon={faRightFromBracket} />
					Logout
				</button>
			{:else}
				<a href="auth" class="btn gap-2">
					<Fa icon={faRightToBracket} />
					Login</a
				>
			{/if}
		</nav>
	</div>
</div>
