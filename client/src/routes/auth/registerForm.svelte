<script lang="ts">
	import { api } from '$lib/api';
	import { userStore } from '$lib/stores';
	import { faUser } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	let newUser = {
		name: '',
		picture: `https://i.pravatar.cc/150?img=${Math.floor(Math.random() * 70)}`,
		account_type: 'customer'
	};
</script>

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
