<script lang="ts">
	import axios from 'axios';
	import { createForm } from 'svelte-forms-lib';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			username: ''
		},
		onSubmit: (values) => {
			axios.post(`account/login/${encodeURIComponent(values.username)}`);
		}
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1 class="text-center mt-32 text-4xl font-serif">
		Aller Anfang ist schwer. Anfangen ist einfach, Beharrlichkeit eine Kunst
	</h1>

	<form class="space-y-2 m-8">
		<label>
			Username
			<input class="input py-2 px-4 mt-2" placeholder="simonhng" />
		</label>

		<button type="submit" class="btn variant-filled w-full">login</button>
	</form>
</section>
