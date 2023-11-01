<script lang="ts">
	import type { Account, Message } from '$lib/types';
	import { userStore } from '$lib/stores';
	import { Avatar } from '@skeletonlabs/skeleton';
	import { formatDate } from '$lib/util/date';

	export let message: Message;
	export let partner: Account;
</script>

{#if message.sender_id !== $userStore.id}
	<div class="grid grid-cols-[auto_1fr] gap-2">
		<Avatar src={partner.picture} width="w-12" />
		<div class="card p-4 variant-soft rounded-tl-none space-y-2">
			<header class="flex justify-between items-center">
				<p class="font-bold">{partner.name}</p>
				<small class="opacity-50">{formatDate(message.created_at)}</small>
			</header>
			<p>{message.text}</p>
		</div>
	</div>
{:else}
	<div class="grid grid-cols-[1fr_auto] gap-2">
		<div class="card p-4 rounded-tr-none space-y-2 bg-primary-200-700-token">
			<header class="flex justify-between items-center">
				<p class="font-bold">{$userStore.name}</p>
				<small class="opacity-50">{formatDate(message.created_at)}</small>
			</header>
			<p>{message.text}</p>
		</div>

		<Avatar src={$userStore?.picture} width="w-12" />
	</div>
{/if}
