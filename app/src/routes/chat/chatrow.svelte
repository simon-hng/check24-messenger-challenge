<script lang="ts">
	import type { Chat } from './+page';
	import { Avatar } from '@skeletonlabs/skeleton';
	export let chat: Chat;
</script>

<li>
	<a class="w-full flex gap-3 p-2" href="chat/{chat.id}">
		<Avatar
			src="https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop"
			width="w-12 h-12"
			rounded="rounded-full"
			class="flex-shrink-0"
		/>
		<div class="overflow-hidden">
			<h3 class="font-semibold text-xl">{chat.name}</h3>
			<p class="text-sm overflow-ellipsis whitespace-nowrap overflow-hidden w-full">
				{chat.last_message}
			</p>
		</div>
		<div class="flex flex-col items-end gap-1 ml-auto">
			<p class="text-sm whitespace-nowrap">10:35 AM</p>
			<div>
				<span class="badge bg-primary-500">1</span>
			</div>
		</div>
	</a>
</li>
