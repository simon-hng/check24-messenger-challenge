<script lang="ts">
	import { createForm } from 'svelte-forms-lib';
	import { userStore } from '$lib/stores';
	import Navbar from '$lib/components/navbar.svelte';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			username: ''
		},

		onSubmit: async (values) => await userStore.login(values.username)
	});
</script>

<Navbar />

{#if $userStore}
	<div class="p-4 space-y-4">
		<p>You're logged in as {$userStore.id}</p>
		<button class="btn variant-filled" on:click={() => userStore.logout()}>Logout</button>
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
