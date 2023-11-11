<script lang="ts">
	import { userStore } from '$lib/stores';
	import { faRightToBracket } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import { createForm } from 'svelte-forms-lib';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			name: ''
		},

		onSubmit: async (values) => await userStore.login(values.name)
	});
</script>

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
