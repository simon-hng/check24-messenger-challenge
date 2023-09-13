<script lang="ts">
	import { Avatar } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeft } from 'svelte-hero-icons';
	export let data;
</script>

<div class="bg-white rounded-b-2xl p-4 flex flex-row items-center gap-3 fixed top-0 left-0 w-full">
	<a href="/conversation">
		<Icon src={ArrowLeft} size="24" />
	</a>
	<div class="flex flex-row gap-2">
		<Avatar
			src="https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop"
			width="w-12 h-12"
			rounded="rounded-full"
			class="flex-shrink-0"
		/>
		<div>
			<h2 class="font-semibold text-xl">NAME</h2>
			<p class="text-sm">online</p>
		</div>
	</div>
</div>

{data.params.id}
