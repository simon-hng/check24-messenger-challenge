<script lang="ts">
	import { userStore } from '$lib/stores';
	import { faRightToBracket } from '@fortawesome/free-solid-svg-icons';
	import type { AxiosError } from 'axios';
	import Fa from 'svelte-fa';
	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';

	const validationSchema = yup.object().shape({
		account_name: yup.string().min(1).required()
	});

	interface LoginOptions extends yup.InferType<typeof validationSchema> {}

	const { form, handleChange, handleSubmit, errors } = createForm({
		initialValues: {
			account_name: '',
			server: undefined
		},
		validationSchema,
		onSubmit: async (user: LoginOptions) => {
			userStore
				.login(user.account_name)
				.catch((err: AxiosError) => ($errors.server = (err.response?.data as string) ?? ''));
		}
	});
</script>

<form class="card mx-8 my-4" on:submit={handleSubmit}>
	<header class="card-header">
		<h2 class="text-2xl mb-3">Log in</h2>
	</header>
	<div class="space-y-2 p-4">
		<label>
			Account name
			<input
				class="input py-2 px-4 mt-2"
				on:change={handleChange}
				bind:value={$form.account_name}
			/>
		</label>
		{#if $errors.account_name}
			<p class="text-error-500">
				{$errors.account_name}
			</p>
		{/if}
	</div>
	<hr class="opacity-50" />
	<footer class="card-footer pt-4 flex justify-between items-center">
		<button type="submit" class="btn variant-filled gap-2">
			<Fa icon={faRightToBracket} />Login
		</button>
		{#if $errors.server}
			<p class="text-error-500">
				{$errors.server}
			</p>
		{/if}
	</footer>
</form>
