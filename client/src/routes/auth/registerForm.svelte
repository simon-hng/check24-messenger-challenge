<script lang="ts">
	import { api } from '$lib/api';
	import { userStore } from '$lib/stores';
	import { faUser } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';

	const validationSchema = yup.object().shape({
		name: yup.string().min(1).required(),
		picture: yup.string().url(),
		account_type: yup.string().oneOf(['Customer', 'ServiceProvider'])
	});

	interface RegisterOptions extends yup.InferType<typeof validationSchema> {}

	const { form, errors, handleChange, handleSubmit } = createForm({
		initialValues: {
			name: '',
			picture: `https://i.pravatar.cc/150?img=${Math.floor(Math.random() * 70)}`,
			account_type: 'Customer',
			server: undefined
		},
		validationSchema,
		onSubmit: async (user: RegisterOptions) => {
			try {
				await api.post('account', user);
				await userStore.login(user.name);
			} catch (err: any) {
				$errors.server = err.response?.data as string;
			}
		}
	});
</script>

<form class="card mx-8" on:submit={handleSubmit}>
	<header class="card-header">
		<h2 class="text-2xl mb-3">Create a new account</h2>
	</header>
	<section class="p-4">
		<label class="label">
			<span>Account name</span>
			<input class="input p-2" type="text" bind:value={$form.name} on:change={handleChange} />
		</label>
		{#if $errors.name}
			<p class="text-error-500">
				{$errors.name}
			</p>
		{/if}

		<label class="label">
			<span>Profile picture</span>
			<input class="input p-2" type="text" bind:value={$form.picture} on:change={handleChange} />
		</label>
		{#if $errors.picture}
			<p class="text-error-500">
				{$errors.picture}
			</p>
		{/if}

		<label class="label">
			<span>Account type</span>
			<select bind:value={$form.account_type} class="select">
				<option value="Customer" selected>Customer</option>
				<option value="ServiceProvider">Service Provider</option>
			</select>
			{#if $errors.account_type}
				<p class="text-error-500">
					{$errors.account_type}
				</p>
			{/if}
		</label>
	</section>
	<hr class="opacity-50" />
	<footer class="card-footer pt-4 flex justify-between items-center">
		<button type="submit" class="btn variant-filled gap-2">
			<Fa icon={faUser} />
			Register</button
		>
		{#if $errors.server}
			<p class="text-error-500">
				{$errors.server}
			</p>
		{/if}
	</footer>
</form>
