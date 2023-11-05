<script lang="ts">
	import { createForm } from 'svelte-forms-lib';
	import { userStore } from '$lib/stores';
	import { Avatar } from '@skeletonlabs/skeleton';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			username: ''
		},

		onSubmit: async (values) => await userStore.login(values.username)
	});
</script>

{#if $userStore}
	<div class="m-8 flex gap-8 flex-col">
		<div class="flex gap-4 items-center">
			<Avatar
				src={$userStore?.picture}
				width="w-12 h-12"
				rounded="rounded-full"
				class="flex-shrink-0"
			/>
			<div>
				<p class="font-bold">{$userStore.name}</p>
				<p class="text-sm">{$userStore.account_type}</p>
			</div>
		</div>
		<a class="btn" href="/conversation">Show conversations</a>
		<button class="btn variant-outline" on:click={() => void userStore.logout()}>Logout</button>
	</div>
{:else}
	<form class="space-y-2 m-8" on:submit={handleSubmit}>
		<label>
			Username
			<input
				class="input py-2 px-4 mt-2"
				placeholder="simonhng"
				on:change={handleChange}
				bind:value={$form.username}
			/>
		</label>

		<button type="submit" class="btn variant-filled w-full">login</button>
	</form>
{/if}
