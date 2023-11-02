<script lang="ts">
	import { createForm } from 'svelte-forms-lib';
	import { userStore } from '$lib/stores';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			username: ''
		},

		onSubmit: async (values) => await userStore.login(values.username)
	});
</script>

{#if $userStore}
	<div class="m-8 flex gap-8 flex-col">
		<button
			class="btn variant-outline"
			on:click={() => {
				userStore.logout();
			}}>Logout</button
		>
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
