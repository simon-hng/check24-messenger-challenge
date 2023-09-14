<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';

	export let message: string;
	export let fromHost = false;

	const bubble = {
		name: 'SenderName',
		timestamp: '12',
		message
	};
</script>

{#if fromHost}
	<div class="grid grid-cols-[auto_1fr] gap-2">
		<Avatar src="https://i.pravatar.cc/?img=1" width="w-12" />
		<div class="card p-4 variant-soft rounded-tl-none space-y-2">
			<header class="flex justify-between items-center">
				<p class="font-bold">{bubble.name}</p>
				<small class="opacity-50">{bubble.timestamp}</small>
			</header>
			<p>{bubble.message}</p>
		</div>
	</div>
{:else}
	<div class="grid grid-cols-[1fr_auto] gap-2">
		<div class="card p-4 rounded-tr-none space-y-2 bg-primary-200-700-token">
			<header class="flex justify-between items-center">
				<p class="font-bold">{bubble.name}</p>
				<small class="opacity-50">{bubble.timestamp}</small>
			</header>
			<p>{bubble.message}</p>
		</div>
		<Avatar src="https://i.pravatar.cc/?img=2" width="w-12" />
	</div>
{/if}
