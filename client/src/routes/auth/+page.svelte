<script lang="ts">
	import { createForm } from 'svelte-forms-lib';
	import { userStore } from '$lib/stores';
	import Navbar from '$lib/components/navbar.svelte';
	import { api } from '$lib/api';
	import Fa from 'svelte-fa';
	import { faRightFromBracket, faRightToBracket, faUser } from '@fortawesome/free-solid-svg-icons';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			name: ''
		},

		onSubmit: async (values) => await userStore.login(values.name)
	});

	let newUser = {
		name: '',
		picture: `https://i.pravatar.cc/150?img=${Math.floor(Math.random() * 70)}`,
		account_type: 'customer'
	};
</script>

<Navbar />

{#if $userStore}
	<div class="p-4 space-y-4">
		<p>You're logged in as {$userStore.id}</p>
		<button class="btn variant-filled gap-2" on:click={() => userStore.logout()}>
			<Fa icon={faRightFromBracket} />
			Logout</button
		>
	</div>
{:else}
	<form class="card mx-8 my-4" on:submit={handleSubmit}>
		<header class="card-header">
			<h2 class="text-2xl mb-3">Log in</h2>
		</header>
		<div class="space-y-2 p-4">
			<label>
				Name
				<input
					class="input py-2 px-4 mt-2"
					placeholder="simonhng"
					on:change={handleChange}
					bind:value={$form.name}
				/>
			</label>
		</div>
		<hr class="opacity-50" />
		<footer class="card-footer pt-4 flex justify-between items-center">
			<button type="submit" class="btn variant-filled gap-2"
				><Fa icon={faRightToBracket} />Login</button
			>
		</footer>
	</form>

	<div class="card mx-8">
		<header class="card-header">
			<h2 class="text-2xl mb-3">Create a new account</h2>
		</header>
		<section class="p-4">
			<label class="label">
				<span>Name</span>
				<input class="input p-2" type="text" placeholder="name" bind:value={newUser.name} />
			</label>

			<label class="label">
				<span>Profile picture</span>
				<input class="input p-2" type="text" placeholder="name" bind:value={newUser.picture} />
			</label>

			<label class="label">
				<span>Account type</span>
				<select bind:value={newUser.account_type} class="select">
					<option value="Customer">Customer</option>
					<option value="ServiceProvider">Service Provider</option>
				</select>
			</label>
		</section>
		<hr class="opacity-50" />
		<footer class="card-footer pt-4 flex justify-between items-center">
			<button
				class="btn variant-filled gap-2"
				on:click={() => {
					api.post('account', newUser).then(() => userStore.login(newUser.name));
				}}
			>
				<Fa icon={faUser} />
				Register</button
			>
		</footer>
	</div>
{/if}
